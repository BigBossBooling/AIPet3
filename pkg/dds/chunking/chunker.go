// pkg/dds/chunking/chunker.go
package chunking

import (
	"crypto/sha256"
	"encoding/hex"
	"fmt"
)

// Chunk represents a piece of content.
type Chunk struct {
	ID   string // CID of the chunk
	Data []byte
	Size int
}

// Manifest describes how to reassemble content from chunks.
type Manifest struct {
	ID        string   // CID of the manifest itself
	ContentID string   // Identifier for the original full content (e.g., hash of content)
	ChunkIDs  []string // Ordered list of chunk CIDs
	TotalSize int64    // Total size of the original content
	// Could include other metadata like encryption details, file type, etc.
}

// Chunker defines the interface for content chunking and manifest generation.
type Chunker interface {
	ChunkContent(content []byte) ([]Chunk, error)
	GenerateManifest(chunks []Chunk, originalContent []byte) (*Manifest, error)
}

// BasicChunker provides a simple fixed-size chunking implementation.
type BasicChunker struct {
	chunkSize int
}

// NewBasicChunker creates a new BasicChunker.
func NewBasicChunker(chunkSize int) *BasicChunker {
	if chunkSize <= 0 {
		chunkSize = 1024 // Default to 1KB if invalid size is given
	}
	return &BasicChunker{chunkSize: chunkSize}
}

// hashData generates a SHA256 hash for given data and returns its hex string.
func hashData(data []byte) string {
	hash := sha256.Sum256(data)
	return hex.EncodeToString(hash[:])
}

// ChunkContent splits content into fixed-size chunks.
func (bc *BasicChunker) ChunkContent(content []byte) ([]Chunk, error) {
	if len(content) == 0 {
		return nil, fmt.Errorf("content cannot be empty")
	}

	var chunks []Chunk
	for i := 0; i < len(content); i += bc.chunkSize {
		end := i + bc.chunkSize
		if end > len(content) {
			end = len(content)
		}
		chunkData := content[i:end]
		chunkID := hashData(chunkData) // Simple hash as chunk ID (CID)
		chunks = append(chunks, Chunk{ID: chunkID, Data: chunkData, Size: len(chunkData)})
	}
	return chunks, nil
}

// GenerateManifest creates a manifest for the given chunks.
func (bc *BasicChunker) GenerateManifest(chunks []Chunk, originalContent []byte) (*Manifest, error) {
	if len(chunks) == 0 {
		return nil, fmt.Errorf("cannot generate manifest for zero chunks")
	}

	var chunkIDs []string
	var totalSize int64
	for _, chunk := range chunks {
		chunkIDs = append(chunkIDs, chunk.ID)
		totalSize += int64(chunk.Size)
	}

	originalContentID := hashData(originalContent) // Hash of the full original content

	// For simplicity, manifest data includes concatenation of chunk IDs and original content ID
	manifestDataStr := originalContentID
	for _, id := range chunkIDs {
		manifestDataStr += id
	}
	manifestID := hashData([]byte(manifestDataStr)) // CID of the manifest

	return &Manifest{
		ID:        manifestID,
		ContentID: originalContentID,
		ChunkIDs:  chunkIDs,
		TotalSize: totalSize,
	}, nil
}

// --- Mock Implementation ---

// MockChunker for testing purposes.
type MockChunker struct {
	ChunkContentFunc     func(content []byte) ([]Chunk, error)
	GenerateManifestFunc func(chunks []Chunk, originalContent []byte) (*Manifest, error)
}

// NewMockChunker creates a new MockChunker.
func NewMockChunker() *MockChunker {
	return &MockChunker{
		// Default mock behavior
		ChunkContentFunc: func(content []byte) ([]Chunk, error) {
			// Simulate creating one or two chunks based on content length for basic testing
			if len(content) == 0 {
				return nil, fmt.Errorf("mock: content cannot be empty")
			}
			numChunks := 1
			if len(content) > 10 { // Arbitrary threshold for multiple chunks
				numChunks = 2
			}
			mockChunks := make([]Chunk, numChunks)
			for i := 0; i < numChunks; i++ {
				mockChunks[i] = Chunk{ID: fmt.Sprintf("mock_chunk_id_%d_for_%x", i+1, content[:min(5, len(content))]), Data: []byte(fmt.Sprintf("mock_data_part_%d", i+1)), Size: 10}
			}
			return mockChunks, nil
		},
		GenerateManifestFunc: func(chunks []Chunk, originalContent []byte) (*Manifest, error) {
			if len(chunks) == 0 {
				return nil, fmt.Errorf("mock: no chunks provided for manifest")
			}
			chunkIDs := make([]string, len(chunks))
			for i, c := range chunks {
				chunkIDs[i] = c.ID
			}
			return &Manifest{
				ID:        fmt.Sprintf("mock_manifest_cid_for_%x", originalContent[:min(5, len(originalContent))]),
				ContentID: fmt.Sprintf("mock_content_id_for_%x", originalContent[:min(5, len(originalContent))]),
				ChunkIDs:  chunkIDs,
				TotalSize: int64(len(originalContent)),
			}, nil
		},
	}
}


func (m *MockChunker) ChunkContent(content []byte) ([]Chunk, error) {
	if m.ChunkContentFunc != nil {
		return m.ChunkContentFunc(content)
	}
	// Fallback to default if specific func not set by test
	return NewMockChunker().ChunkContentFunc(content)
}

func (m *MockChunker) GenerateManifest(chunks []Chunk, originalContent []byte) (*Manifest, error) {
	if m.GenerateManifestFunc != nil {
		return m.GenerateManifestFunc(chunks, originalContent)
	}
	// Fallback to default if specific func not set by test
	return NewMockChunker().GenerateManifestFunc(chunks, originalContent)
}

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}
