// pkg/dds/storage/storage.go
package storage

import (
	"digisocialblock/pkg/dds/chunking" // Referencing the chunking package
	"fmt"
	"sync"
)

// Storage defines the interface for storing and retrieving chunks and manifests.
type Storage interface {
	StoreChunk(chunk chunking.Chunk) error
	GetChunk(chunkID string) (chunking.Chunk, error)
	StoreManifest(manifest *chunking.Manifest) error
	GetManifest(manifestID string) (*chunking.Manifest, error)
	// Future methods: HasChunk(chunkID string) bool, DeleteChunk(chunkID string) error, etc.
}

// InMemoryStorage provides a basic in-memory implementation of the Storage interface.
// This is suitable for testing and simple scenarios.
type InMemoryStorage struct {
	chunks    map[string]chunking.Chunk
	manifests map[string]*chunking.Manifest
	mu        sync.RWMutex // For thread-safe access
}

// NewInMemoryStorage creates a new InMemoryStorage instance.
func NewInMemoryStorage() *InMemoryStorage {
	return &InMemoryStorage{
		chunks:    make(map[string]chunking.Chunk),
		manifests: make(map[string]*chunking.Manifest),
	}
}

// StoreChunk adds a chunk to the in-memory store.
func (s *InMemoryStorage) StoreChunk(chunk chunking.Chunk) error {
	s.mu.Lock()
	defer s.mu.Unlock()

	if _, exists := s.chunks[chunk.ID]; exists {
		// Depending on strategy, either error or ignore. For simplicity, we'll overwrite.
		// fmt.Printf("Storage: Chunk %s already exists. Overwriting.\n", chunk.ID)
	}
	s.chunks[chunk.ID] = chunk
	// fmt.Printf("Storage: Stored chunk %s\n", chunk.ID)
	return nil
}

// GetChunk retrieves a chunk from the in-memory store by its ID.
func (s *InMemoryStorage) GetChunk(chunkID string) (chunking.Chunk, error) {
	s.mu.RLock()
	defer s.mu.RUnlock()

	chunk, exists := s.chunks[chunkID]
	if !exists {
		return chunking.Chunk{}, fmt.Errorf("chunk with ID %s not found in in-memory storage", chunkID)
	}
	return chunk, nil
}

// StoreManifest adds a manifest to the in-memory store.
func (s *InMemoryStorage) StoreManifest(manifest *chunking.Manifest) error {
	s.mu.Lock()
	defer s.mu.Unlock()

	if _, exists := s.manifests[manifest.ID]; exists {
		// Overwriting for simplicity
		// fmt.Printf("Storage: Manifest %s already exists. Overwriting.\n", manifest.ID)
	}
	s.manifests[manifest.ID] = manifest
	// fmt.Printf("Storage: Stored manifest %s\n", manifest.ID)
	return nil
}

// GetManifest retrieves a manifest from the in-memory store by its ID.
func (s *InMemoryStorage) GetManifest(manifestID string) (*chunking.Manifest, error) {
	s.mu.RLock()
	defer s.mu.RUnlock()

	manifest, exists := s.manifests[manifestID]
	if !exists {
		return nil, fmt.Errorf("manifest with ID %s not found in in-memory storage", manifestID)
	}
	return manifest, nil
}

// --- Mock Implementation ---

// MockStorage for testing purposes.
type MockStorage struct {
	StoreChunkFunc    func(chunk chunking.Chunk) error
	GetChunkFunc      func(chunkID string) (chunking.Chunk, error)
	StoreManifestFunc func(manifest *chunking.Manifest) error
	GetManifestFunc   func(manifestID string) (*chunking.Manifest, error)
	StoredChunks      map[string]chunking.Chunk    // To inspect after calls
	StoredManifests   map[string]*chunking.Manifest // To inspect after calls
}

// NewMockStorage creates a new MockStorage.
func NewMockStorage() *MockStorage {
	ms := &MockStorage{
		StoredChunks:    make(map[string]chunking.Chunk),
		StoredManifests: make(map[string]*chunking.Manifest),
	}
	// Default mock behavior:
	// Store functions will just store in the maps if not overridden by a test.
	// Get functions will retrieve from the maps if not overridden by a test.
	ms.StoreChunkFunc = func(chunk chunking.Chunk) error {
		// Removed ms.mu.Lock() and ms.mu.Unlock()
		if ms.StoredChunks == nil {
			ms.StoredChunks = make(map[string]chunking.Chunk)
		}
		ms.StoredChunks[chunk.ID] = chunk
		return nil
	}
	ms.GetChunkFunc = func(chunkID string) (chunking.Chunk, error) {
		// Removed ms.mu.RLock() and ms.mu.RUnlock()
		if chunk, ok := ms.StoredChunks[chunkID]; ok {
			return chunk, nil
		}
		return chunking.Chunk{}, fmt.Errorf("mock storage: GetChunkFunc: chunk %s not found", chunkID)
	}
	ms.StoreManifestFunc = func(manifest *chunking.Manifest) error {
		// Removed ms.mu.Lock() and ms.mu.Unlock()
		if ms.StoredManifests == nil {
			ms.StoredManifests = make(map[string]*chunking.Manifest)
		}
		ms.StoredManifests[manifest.ID] = manifest
		return nil
	}
	ms.GetManifestFunc = func(manifestID string) (*chunking.Manifest, error) {
		// Removed ms.mu.RLock() and ms.mu.RUnlock()
		if manifest, ok := ms.StoredManifests[manifestID]; ok {
			return manifest, nil
		}
		return nil, fmt.Errorf("mock storage: GetManifestFunc: manifest %s not found", manifestID)
	}
	return ms
}

// StoreChunk calls the configured StoreChunkFunc.
func (m *MockStorage) StoreChunk(chunk chunking.Chunk) error {
	return m.StoreChunkFunc(chunk)
}

// GetChunk calls the configured GetChunkFunc.
func (m *MockStorage) GetChunk(chunkID string) (chunking.Chunk, error) {
	return m.GetChunkFunc(chunkID)
}

// StoreManifest calls the configured StoreManifestFunc.
func (m *MockStorage) StoreManifest(manifest *chunking.Manifest) error {
	return m.StoreManifestFunc(manifest)
}

// GetManifest calls the configured GetManifestFunc.
func (m *MockStorage) GetManifest(manifestID string) (*chunking.Manifest, error) {
	return m.GetManifestFunc(manifestID)
}
