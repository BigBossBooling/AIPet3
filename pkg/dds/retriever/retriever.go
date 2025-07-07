// pkg/dds/retriever/retriever.go
package retriever

import (
	"digisocialblock/pkg/dds/chunking" // Assuming Manifest is defined here
	"errors"                           // Added for predefined error
	"fmt"
)

// ErrSimulatedRetriever is a predefined error for mock retriever simulations.
var ErrSimulatedRetriever = errors.New("simulated retriever error")

// Retriever defines the interface for components responsible for
// fetching content manifests and chunks from the DDS network (or a local cache).
type Retriever interface {
	// FetchManifest retrieves a content manifest by its CID.
	// In a real system, this might involve network calls to a DHT or trusted peers.
	FetchManifest(manifestCID string) (*chunking.Manifest, error)

	// FetchChunk retrieves a single content chunk by its CID.
	// In a real system, this would involve finding and fetching the chunk from peers.
	FetchChunk(chunkCID string) (chunking.Chunk, error)
}

// MockRetriever provides a basic mock implementation for testing.
// It can be seeded with data or configured to return errors.
type MockRetriever struct {
	KnownManifests  map[string]*chunking.Manifest
	KnownChunks     map[string]chunking.Chunk
	SimulateError   bool
	ErrorToReturn   error
	FetchManifestFunc func(manifestCID string) (*chunking.Manifest, error)
	FetchChunkFunc    func(chunkCID string) (chunking.Chunk, error)
}

// NewMockRetriever creates a new MockRetriever instance.
func NewMockRetriever() *MockRetriever {
	return &MockRetriever{
		KnownManifests: make(map[string]*chunking.Manifest),
		KnownChunks:    make(map[string]chunking.Chunk),
	}
}

// FetchManifest simulates fetching a manifest.
func (mr *MockRetriever) FetchManifest(manifestCID string) (*chunking.Manifest, error) {
	if mr.FetchManifestFunc != nil {
		return mr.FetchManifestFunc(manifestCID)
	}
	if mr.SimulateError {
		if mr.ErrorToReturn != nil {
			return nil, mr.ErrorToReturn
		}
		return nil, ErrSimulatedRetriever // Use predefined error
	}
	manifest, ok := mr.KnownManifests[manifestCID]
	if !ok {
		return nil, fmt.Errorf("mock retriever: manifest %s not found", manifestCID)
	}
	return manifest, nil
}

// FetchChunk simulates fetching a chunk.
func (mr *MockRetriever) FetchChunk(chunkCID string) (chunking.Chunk, error) {
	if mr.FetchChunkFunc != nil {
		return mr.FetchChunkFunc(chunkCID)
	}
	if mr.SimulateError {
		if mr.ErrorToReturn != nil {
			return chunking.Chunk{}, mr.ErrorToReturn
		}
		return chunking.Chunk{}, ErrSimulatedRetriever // Use predefined error
	}
	chunk, ok := mr.KnownChunks[chunkCID]
	if !ok {
		return chunking.Chunk{}, fmt.Errorf("mock retriever: chunk %s not found", chunkCID)
	}
	return chunk, nil
}

// AddManifest allows tests to populate the mock retriever's known manifests.
func (mr *MockRetriever) AddManifest(manifest *chunking.Manifest) {
	if mr.KnownManifests == nil {
		mr.KnownManifests = make(map[string]*chunking.Manifest)
	}
	mr.KnownManifests[manifest.ID] = manifest
}

// AddChunk allows tests to populate the mock retriever's known chunks.
func (mr *MockRetriever) AddChunk(chunk chunking.Chunk) {
	if mr.KnownChunks == nil {
		mr.KnownChunks = make(map[string]chunking.Chunk)
	}
	mr.KnownChunks[chunk.ID] = chunk
}
