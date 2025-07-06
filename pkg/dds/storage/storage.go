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
	return &MockStorage{
		StoredChunks:    make(map[string]chunking.Chunk),
		StoredManifests: make(map[string]*chunking.Manifest),
		// Default mock behavior
		StoreChunkFunc: func(chunk chunking.Chunk) error {
			// Access StoredChunks from the outer scope if needed or handle internally
			// For now, just a placeholder for successful storage
			return nil
		},
		GetChunkFunc: func(chunkID string) (chunking.Chunk, error) {
			return chunking.Chunk{ID: chunkID, Data: []byte("mock_chunk_data")}, nil
		},
		StoreManifestFunc: func(manifest *chunking.Manifest) error {
			return nil
		},
		GetManifestFunc: func(manifestID string) (*chunking.Manifest, error) {
			return &chunking.Manifest{ID: manifestID, ChunkIDs: []string{"mock_cid1"}}, nil
		},
	}
}

func (m *MockStorage) StoreChunk(chunk chunking.Chunk) error {
	if m.StoreChunkFunc != nil {
		err := m.StoreChunkFunc(chunk)
		if err == nil { // If the custom func doesn't error, record the chunk
			if m.StoredChunks == nil { // Ensure map is initialized
				m.StoredChunks = make(map[string]chunking.Chunk)
			}
			m.StoredChunks[chunk.ID] = chunk
		}
		return err
	}
	// Fallback to default (and record) if specific func not set by test
	if m.StoredChunks == nil {
		m.StoredChunks = make(map[string]chunking.Chunk)
	}
	m.StoredChunks[chunk.ID] = chunk
	return nil
}

func (m *MockStorage) GetChunk(chunkID string) (chunking.Chunk, error) {
	if m.GetChunkFunc != nil {
		return m.GetChunkFunc(chunkID)
	}
	// Fallback to default if specific func not set by test
	if chunk, ok := m.StoredChunks[chunkID]; ok {
		return chunk, nil
	}
	return chunking.Chunk{}, fmt.Errorf("mock: chunk %s not found", chunkID)
}

func (m *MockStorage) StoreManifest(manifest *chunking.Manifest) error {
	if m.StoreManifestFunc != nil {
		err := m.StoreManifestFunc(manifest)
		if err == nil {
			if m.StoredManifests == nil {
				m.StoredManifests = make(map[string]*chunking.Manifest)
			}
			m.StoredManifests[manifest.ID] = manifest
		}
		return err
	}
	if m.StoredManifests == nil {
		m.StoredManifests = make(map[string]*chunking.Manifest)
	}
	m.StoredManifests[manifest.ID] = manifest
	return nil
}

func (m *MockStorage) GetManifest(manifestID string) (*chunking.Manifest, error) {
	if m.GetManifestFunc != nil {
		return m.GetManifestFunc(manifestID)
	}
	if manifest, ok := m.StoredManifests[manifestID]; ok {
		return manifest, nil
	}
	return nil, fmt.Errorf("mock: manifest %s not found", manifestID)
}
