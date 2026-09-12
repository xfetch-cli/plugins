// Command wasm-pacman is an xfetch WebAssembly plugin that reports package
// counts from the local pacman database.
//
// It demonstrates the Go side of the host bridge: `//go:wasmimport` for the
// JSON host_call entry point and `//go:wasmexport` for the allocator the host
// uses to place responses. The manifest only allowlists the `pacman` program;
// no shell is involved.
package main

import (
	"bufio"
	"encoding/base64"
	"encoding/json"
	"fmt"
	"os"
	"strings"
	"unsafe"
)

//go:wasmimport xfetch host_call
func hostCall(opPtr, opLen, argsPtr, argsLen uint32) uint64

// pinned keeps the response buffer reachable while the host writes into it.
var pinned []byte

//go:wasmexport xfetch_alloc
func xfetchAlloc(size uint32) uint32 {
	if size == 0 {
		return 0
	}
	pinned = make([]byte, size)
	return uint32(uintptr(unsafe.Pointer(&pinned[0])))
}

//go:wasmexport xfetch_free
func xfetchFree(ptr, size uint32) {}

// request mirrors the xfetch plugin protocol v1.
type request struct {
	Version uint32 `json:"version"`
	Kind    string `json:"kind"`
	Args    struct {
		Samples int `json:"samples"`
	} `json:"args"`
}

// response is the info-provider payload.
type response struct {
	Lines []string `json:"lines"`
}

func main() {
	var req request
	_ = json.NewDecoder(os.Stdin).Decode(&req)

	total, err := countLines("-Qq")
	if err != nil {
		write(response{Lines: []string{fmt.Sprintf("packages: %s", err)}})
		return
	}
	foreign, _ := countLines("-Qqm")
	native := total - foreign

	lines := []string{
		fmt.Sprintf("packages: %d installed (%d repo, %d AUR/foreign)", total, native, foreign),
	}

	samples := req.Args.Samples
	if samples <= 0 {
		samples = 3
	}
	if names, err := firstLines([]string{"-Qqm"}, samples); err == nil && len(names) > 0 {
		lines = append(lines, "foreign: "+strings.Join(names, ", "))
	}
	write(response{Lines: lines})
}

// callHost dispatches one host operation and returns its JSON value.
func callHost(op string, args any) (json.RawMessage, error) {
	argBytes, err := json.Marshal(args)
	if err != nil {
		return nil, err
	}
	opBytes := []byte(op)

	packed := hostCall(
		uint32(uintptr(unsafe.Pointer(&opBytes[0]))), uint32(len(opBytes)),
		uint32(uintptr(unsafe.Pointer(&argBytes[0]))), uint32(len(argBytes)),
	)
	if packed == 0 {
		return nil, fmt.Errorf("host returned no buffer")
	}

	ptr := uint32(packed & 0xffffffff)
	length := uint32(packed >> 32)
	data := unsafe.Slice((*byte)(unsafe.Pointer(uintptr(ptr))), length)
	raw := append([]byte(nil), data...)
	pinned = nil

	var envelope struct {
		OK    bool            `json:"ok"`
		Value json.RawMessage `json:"value"`
		Error struct {
			Kind    string `json:"kind"`
			Message string `json:"message"`
		} `json:"error"`
	}
	if err := json.Unmarshal(raw, &envelope); err != nil {
		return nil, err
	}
	if !envelope.OK {
		return nil, fmt.Errorf("%s: %s", envelope.Error.Kind, envelope.Error.Message)
	}
	return envelope.Value, nil
}

// runPacman executes an allowlisted pacman query and returns its stdout.
func runPacman(args ...string) (string, error) {
	value, err := callHost("exec", map[string]any{
		"program":    "pacman",
		"args":       args,
		"timeout_ms": 8000,
	})
	if err != nil {
		return "", err
	}

	var result struct {
		Code   int    `json:"code"`
		Stdout string `json:"stdout_base64"`
	}
	if err := json.Unmarshal(value, &result); err != nil {
		return "", err
	}
	if result.Code != 0 {
		return "", fmt.Errorf("pacman exited with code %d", result.Code)
	}

	decoded, err := base64.StdEncoding.DecodeString(result.Stdout)
	if err != nil {
		return "", err
	}
	return string(decoded), nil
}

// countLines returns the number of non-empty lines in a pacman query.
func countLines(args ...string) (int, error) {
	output, err := runPacman(args...)
	if err != nil {
		return 0, err
	}
	count := 0
	for _, line := range strings.Split(output, "\n") {
		if strings.TrimSpace(line) != "" {
			count++
		}
	}
	return count, nil
}

// firstLines returns the first non-empty lines of a pacman query.
func firstLines(args []string, limit int) ([]string, error) {
	output, err := runPacman(args...)
	if err != nil {
		return nil, err
	}
	var lines []string
	scanner := bufio.NewScanner(strings.NewReader(output))
	for scanner.Scan() && len(lines) < limit {
		line := strings.TrimSpace(scanner.Text())
		if line != "" {
			lines = append(lines, line)
		}
	}
	return lines, scanner.Err()
}

func write(res response) {
	encoded, err := json.Marshal(res)
	if err != nil {
		os.Exit(1)
	}
	os.Stdout.Write(encoded)
}
