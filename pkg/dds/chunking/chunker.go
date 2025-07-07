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
	// Default mock behavior using hashData for consistency
	defaultChunkContentFunc := func(content []byte) ([]Chunk, error) {
		if len(content) == 0 {
			return nil, fmt.Errorf("mock chunker: content cannot be empty")
		}
		// Simulate BasicChunker logic with a fixed mock chunk size for testing
		mockChunkSize := 10 // Can be parameterized if needed for more complex tests
		var mockChunks []Chunk
		for i := 0; i < len(content); i += mockChunkSize {
			end := i + mockChunkSize
			if end > len(content) {
				end = len(content)
			}
			chunkData := content[i:end]
			// Use actual hashData for ID, consistent with BasicChunker and retriever's expectations
			chunkID := hashData(chunkData)
			mockChunks = append(mockChunks, Chunk{ID: chunkID, Data: chunkData, Size: len(chunkData)})
		}
		return mockChunks, nil
	}

	defaultGenerateManifestFunc := func(chunks []Chunk, originalContent []byte) (*Manifest, error) {
		if len(chunks) == 0 {
			return nil, fmt.Errorf("mock chunker: no chunks provided for manifest")
		}
		var chunkIDs []string
		var totalSize int64
		for _, ch := range chunks {
			chunkIDs = append(chunkIDs, ch.ID)
			totalSize += int64(ch.Size)
		}

		contentID := hashData(originalContent)

		manifestDataStr := contentID
		for _, id := range chunkIDs {
			manifestDataStr += id
		}
		manifestID := hashData([]byte(manifestDataStr))

		return &Manifest{
			ID:        manifestID,
			ContentID: contentID,
			ChunkIDs:  chunkIDs,
			TotalSize: totalSize,
		}, nil
	}

	return &MockChunker{
		ChunkContentFunc:     defaultChunkContentFunc,
		GenerateManifestFunc: defaultGenerateManifestFunc,
	}
}

func (m *MockChunker) ChunkContent(content []byte) ([]Chunk, error) {
	if m.ChunkContentFunc != nil { // Allow overriding default mock behavior in specific tests
		return m.ChunkContentFunc(content)
	}
	// This recursive call to NewMockChunker().ChunkContentFunc was problematic.
	// The default funcs should be assigned at construction and called directly.
	// However, the current structure of MockChunker already stores these functions.
	// So, if ChunkContentFunc is not overridden by a test, it *is* the default one.
	// The issue was if a test set it to nil, it would infinitely recurse.
	// The fix is to ensure NewMockChunker assigns non-nil default functions.
	// And if a test wants to disable it, it should provide a func that returns an error.
	return m.ChunkContentFunc(content) // Call the stored function (default or overridden)
}

func (m *MockChunker) GenerateManifest(chunks []Chunk, originalContent []byte) (*Manifest, error) {
	if m.GenerateManifestFunc != nil { // Allow overriding
		return m.GenerateManifestFunc(chunks, originalContent)
	}
	return m.GenerateManifestFunc(chunks, originalContent) // Call the stored function
}

// min function was here, ensure it's either kept if used by other parts of this file
// or removed if it was only for the old mock logic.
// It seems it was only for the old mock logic's fmt.Sprintf.
// func min(a, b int) int {
// 	if a < b {
// 		return a
// 	}
// 	return b
// }

// min function, if needed, should be defined or imported properly.
// For now, removing it as the new mock logic doesn't use it.
// If hashData or other parts of this package need it, it should be reinstated or handled.
// The hashData function does not use min. The BasicChunker does not use min.
// It was solely for the old mock's fmt.Sprintf.

// Helper min function if it were needed elsewhere (it's not for current code)
/*
func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}
*/
// Removed unused min function from the end of the file
