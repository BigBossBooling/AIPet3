// core/content/publisher_test.go
package content_test

import (
	"errors"
	// "fmt" // Removed unused import
	"testing"
	"digisocialblock/core/content"
	"digisocialblock/pkg/dds/chunking"
	// "digisocialblock/pkg/dds/storage" // Removed unused import
	// "digisocialblock/pkg/dds/originator" // Removed unused import
)

// --- Expected Error Instances for Mocks ---
var (
	errMockChunking        = errors.New("mock chunking error")
	errMockManifest        = errors.New("mock manifest error")
	errMockStoreChunk      = errors.New("mock store chunk error")
	errMockStoreManifest   = errors.New("mock store manifest error")
	errMockAdvertise       = errors.New("mock advertise error")
)

// --- Mocks for DDS Interfaces ---

// MockChunker implements chunking.Chunker for testing.
type MockChunker struct {
	ChunkContentFunc   func(content []byte) ([]chunking.Chunk, error)
	GenerateManifestFunc func(chunks []chunking.Chunk, originalContent []byte) (*chunking.Manifest, error)
}

func (m *MockChunker) ChunkContent(content []byte) ([]chunking.Chunk, error) {
	if m.ChunkContentFunc != nil {
		return m.ChunkContentFunc(content)
	}
	return []chunking.Chunk{
		{ID: "mock_chunk_id_1", Data: []byte("mock_data_1")},
		{ID: "mock_chunk_id_2", Data: []byte("mock_data_2")},
	}, nil
}

func (m *MockChunker) GenerateManifest(chunks []chunking.Chunk, originalContent []byte) (*chunking.Manifest, error) {
	if m.GenerateManifestFunc != nil {
		return m.GenerateManifestFunc(chunks, originalContent)
	}
	return &chunking.Manifest{
		ID:        "mock_manifest_cid",
		ContentID: "mock_original_content_id",
		ChunkIDs:  []string{"mock_chunk_id_1", "mock_chunk_id_2"},
		TotalSize: int64(len(originalContent)), // Corrected type conversion
	}, nil
}

// MockStorage implements storage.Storage for testing.
type MockStorage struct {
	StoreChunkFunc   func(chunk chunking.Chunk) error
	GetChunkFunc     func(chunkID string) (chunking.Chunk, error)
	StoreManifestFunc func(manifest *chunking.Manifest) error
	GetManifestFunc   func(manifestID string) (*chunking.Manifest, error)
}

func (m *MockStorage) StoreChunk(chunk chunking.Chunk) error {
	if m.StoreChunkFunc != nil {
		return m.StoreChunkFunc(chunk)
	}
	return nil // Simulate success
}

func (m *MockStorage) GetChunk(chunkID string) (chunking.Chunk, error) {
	if m.GetChunkFunc != nil {
		return m.GetChunkFunc(chunkID)
	}
	return chunking.Chunk{}, errors.New("not implemented")
}

func (m *MockStorage) StoreManifest(manifest *chunking.Manifest) error {
	if m.StoreManifestFunc != nil {
		return m.StoreManifestFunc(manifest)
	}
	return nil // Simulate success
}

func (m *MockStorage) GetManifest(manifestID string) (*chunking.Manifest, error) {
	if m.GetManifestFunc != nil {
		return m.GetManifestFunc(manifestID)
	}
	return nil, errors.New("not implemented")
}

// MockOriginator implements originator.Originator for testing.
type MockOriginator struct {
	AdvertiseContentFunc func(manifestID string) error
	AdvertisedIDs        []string // To check if advertise was called
}

func (m *MockOriginator) AdvertiseContent(manifestID string) error {
	if m.AdvertiseContentFunc != nil {
		return m.AdvertiseContentFunc(manifestID)
	}
	m.AdvertisedIDs = append(m.AdvertisedIDs, manifestID)
	return nil // Simulate success
}

// --- Unit Tests for ContentPublisher ---

func TestPublishContent_Success(t *testing.T) {
	mockChunker := &MockChunker{}
	mockStorage := &MockStorage{}
	mockOriginator := &MockOriginator{}

	publisher := content.NewContentPublisher(mockChunker, mockStorage, mockOriginator)

	testContent := []byte("This is some test content for publishing.")
	expectedCID := "mock_manifest_cid" // From MockChunker's default

	cid, err := publisher.PublishContent(testContent)
	if err != nil {
		t.Fatalf("PublishContent failed unexpectedly: %v", err)
	}

	if cid != expectedCID {
		t.Errorf("Expected CID %s, got %s", expectedCID, cid)
	}

	if len(mockOriginator.AdvertisedIDs) != 1 || mockOriginator.AdvertisedIDs[0] != expectedCID {
		t.Errorf("Content was not advertised correctly. Advertised IDs: %v", mockOriginator.AdvertisedIDs)
	}
}

func TestPublishContent_ChunkingFails(t *testing.T) {
	mockChunker := &MockChunker{
		ChunkContentFunc: func(content []byte) ([]chunking.Chunk, error) {
			return nil, errMockChunking // Use defined error
		},
	}
	mockStorage := &MockStorage{}
	mockOriginator := &MockOriginator{}

	publisher := content.NewContentPublisher(mockChunker, mockStorage, mockOriginator)

	_, err := publisher.PublishContent([]byte("test"))
	if err == nil {
		t.Fatal("Expected an error from chunking, but got none")
	}
	// Check if the specific error is in the chain
	if !errors.Is(err, errMockChunking) {
		t.Errorf("Expected error chain to include '%v', got '%v'", errMockChunking, err)
	}
}

func TestPublishContent_GenerateManifestFails(t *testing.T) {
	mockChunker := &MockChunker{
		GenerateManifestFunc: func(chunks []chunking.Chunk, originalContent []byte) (*chunking.Manifest, error) {
			return nil, errMockManifest // Use defined error
		},
	}
	mockStorage := &MockStorage{}
	mockOriginator := &MockOriginator{}

	publisher := content.NewContentPublisher(mockChunker, mockStorage, mockOriginator)

	_, err := publisher.PublishContent([]byte("test"))
	if err == nil {
		t.Fatal("Expected an error from manifest generation, but got none")
	}
	if !errors.Is(err, errMockManifest) {
		t.Errorf("Expected error chain to include '%v', got '%v'", errMockManifest, err)
	}
}

func TestPublishContent_StoreChunkFails(t *testing.T) {
	mockChunker := &MockChunker{
		ChunkContentFunc: func(content []byte) ([]chunking.Chunk, error) {
			return []chunking.Chunk{{ID: "c1", Data: []byte("d1")}}, nil
		},
	}
	mockStorage := &MockStorage{
		StoreChunkFunc: func(chunk chunking.Chunk) error {
			return errMockStoreChunk // Use defined error
		},
	}
	mockOriginator := &MockOriginator{}

	publisher := content.NewContentPublisher(mockChunker, mockStorage, mockOriginator)

	_, err := publisher.PublishContent([]byte("test"))
	if err == nil {
		t.Fatal("Expected an error from storing chunk, but got none")
	}
	if !errors.Is(err, errMockStoreChunk) {
		t.Errorf("Expected error chain to include '%v', got '%v'", errMockStoreChunk, err)
	}
}

func TestPublishContent_StoreManifestFails(t *testing.T) {
	mockChunker := &MockChunker{} // Uses default success
	mockStorage := &MockStorage{
		StoreManifestFunc: func(manifest *chunking.Manifest) error {
			return errMockStoreManifest // Use defined error
		},
	}
	mockOriginator := &MockOriginator{}

	publisher := content.NewContentPublisher(mockChunker, mockStorage, mockOriginator)

	_, err := publisher.PublishContent([]byte("test"))
	if err == nil {
		t.Fatal("Expected an error from storing manifest, but got none")
	}
	if !errors.Is(err, errMockStoreManifest) {
		t.Errorf("Expected error chain to include '%v', got '%v'", errMockStoreManifest, err)
	}
}

func TestPublishContent_AdvertiseFails(t *testing.T) {
	mockChunker := &MockChunker{}
	mockStorage := &MockStorage{}
	mockOriginator := &MockOriginator{
		AdvertiseContentFunc: func(manifestID string) error {
			return errMockAdvertise // Use defined error
		},
	}

	publisher := content.NewContentPublisher(mockChunker, mockStorage, mockOriginator)

	_, err := publisher.PublishContent([]byte("test"))
	if err == nil {
		t.Fatal("Expected an error from advertising, but got none")
	}
	if !errors.Is(err, errMockAdvertise) {
		t.Errorf("Expected error chain to include '%v', got '%v'", errMockAdvertise, err)
	}
}
