// core/content/retriever_test.go
package content_test

import (
	"crypto/sha256"
	"digisocialblock/core/content"
	"digisocialblock/pkg/dds/chunking"
	"digisocialblock/pkg/dds/retriever" // Using the mock from pkg/dds/retriever
	"encoding/hex"
	"errors"
	"fmt"
	"strings"
	"testing"
)

// Helper to create a consistent hash for content, matching chunker's internal logic
func hashTestData(data []byte) string {
	hash := sha256.Sum256(data)
	return hex.EncodeToString(hash[:])
}

// --- Unit Tests for ContentRetriever ---

func TestRetrieveContent_Success(t *testing.T) {
	originalContent := []byte("Hello, decentralized world! This is a test.")

	// Simulate chunking process to prepare data for the mock retriever
	testChunker := chunking.NewBasicChunker(10) // Same chunk size as in main integration test
	chunks, _ := testChunker.ChunkContent(originalContent)
	manifest, _ := testChunker.GenerateManifest(chunks, originalContent)

	mockRetriever := retriever.NewMockRetriever()
	mockRetriever.AddManifest(manifest)
	for _, ch := range chunks {
		mockRetriever.AddChunk(ch)
	}

	contentRetriever := content.NewContentRetriever(mockRetriever)

	retrievedData, err := contentRetriever.RetrieveContent(manifest.ID)
	if err != nil {
		t.Fatalf("RetrieveContent failed unexpectedly: %v", err)
	}

	if string(retrievedData) != string(originalContent) {
		t.Errorf("Retrieved content mismatch. Expected '%s', got '%s'", string(originalContent), string(retrievedData))
	}
}

func TestRetrieveContent_ManifestNotFound(t *testing.T) {
	mockRetriever := retriever.NewMockRetriever() // Empty retriever
	contentRetriever := content.NewContentRetriever(mockRetriever)

	_, err := contentRetriever.RetrieveContent("non_existent_manifest_cid")
	if err == nil {
		t.Fatal("Expected an error when manifest is not found, but got nil")
	}
	expectedErrorMsg := "mock retriever: manifest non_existent_manifest_cid not found"
	if !strings.Contains(err.Error(), expectedErrorMsg) {
		t.Errorf("Expected error message to contain '%s', got '%v'", expectedErrorMsg, err)
	}
}

func TestRetrieveContent_ChunkNotFound(t *testing.T) {
	originalContent := []byte("Chunk missing test")
	testChunker := chunking.NewBasicChunker(5)
	chunks, _ := testChunker.ChunkContent(originalContent)
	manifest, _ := testChunker.GenerateManifest(chunks, originalContent)

	mockRetriever := retriever.NewMockRetriever()
	mockRetriever.AddManifest(manifest)
	// Intentionally do not add all chunks
	if len(chunks) > 0 {
		mockRetriever.AddChunk(chunks[0]) // Add only the first chunk
	}
	if len(chunks) == 0 {
		t.Skip("Skipping test: no chunks generated for 'Chunk missing test'") // Should not happen with content
	}


	contentRetriever := content.NewContentRetriever(mockRetriever)

	missingChunkID := ""
	if len(chunks) > 1 {
		missingChunkID = chunks[1].ID
	} else if len(chunks) == 1 && manifest.TotalSize > 0 {
		if len(manifest.ChunkIDs) > 1 {
             missingChunkID = manifest.ChunkIDs[1]
        } else {
            mockRetriever.KnownChunks = make(map[string]chunking.Chunk)
            missingChunkID = manifest.ChunkIDs[0]
        }
	}


	_, err := contentRetriever.RetrieveContent(manifest.ID)
	if err == nil {
		t.Fatal("Expected an error when a chunk is not found, but got nil")
	}

	if missingChunkID == "" && len(manifest.ChunkIDs) > 0 {
	} else if missingChunkID == "" && len(manifest.ChunkIDs) == 0 && manifest.TotalSize >0 {
		t.Log("Manifest has non-zero size but no chunk IDs, RetrieveContent should ideally error earlier.")
		return
	}


	if !strings.Contains(err.Error(), "failed to fetch chunk") {
		t.Errorf("Expected error message to contain 'failed to fetch chunk', got '%v'", err)
	}
	if missingChunkID!="" && !strings.Contains(err.Error(), missingChunkID) {
         t.Errorf("Expected error message to contain missing chunk ID '%s', got '%v'", missingChunkID, err)
    }
}

func TestRetrieveContent_ChunkIntegrityMismatch(t *testing.T) {
	originalContent := []byte("Corrupted chunk data test")
	testChunker := chunking.NewBasicChunker(10)
	chunks, _ := testChunker.ChunkContent(originalContent)
	manifest, _ := testChunker.GenerateManifest(chunks, originalContent)

	mockRetriever := retriever.NewMockRetriever()
	mockRetriever.AddManifest(manifest)

	if len(chunks) == 0 {
		t.Skip("Skipping test as no chunks were generated.")
	}

	corruptedChunk := chunks[0]
	corruptedChunk.Data = []byte("this is not the original data")

	mockRetriever.AddChunk(corruptedChunk)
	for i := 1; i < len(chunks); i++ {
		mockRetriever.AddChunk(chunks[i])
	}

	contentRetriever := content.NewContentRetriever(mockRetriever)
	_, err := contentRetriever.RetrieveContent(manifest.ID)

	if err == nil {
		t.Fatal("Expected an error due to chunk integrity mismatch, but got nil")
	}
	expectedErrorMsg := fmt.Sprintf("integrity check failed for chunk %s", corruptedChunk.ID)
	if !strings.Contains(err.Error(), expectedErrorMsg) {
		t.Errorf("Expected error message to contain '%s', got '%v'", expectedErrorMsg, err)
	}
}


func TestRetrieveContent_OverallContentIntegrityMismatch(t *testing.T) {
	originalContent := []byte("This content will be 'corrupted' by a bad manifest ContentID.")
	testChunker := chunking.NewBasicChunker(10)
	chunks, _ := testChunker.ChunkContent(originalContent)
	manifest, _ := testChunker.GenerateManifest(chunks, originalContent)

	manifest.ContentID = hashTestData([]byte("completely different content"))

	mockRetriever := retriever.NewMockRetriever()
	mockRetriever.AddManifest(manifest)
	for _, ch := range chunks {
		mockRetriever.AddChunk(ch)
	}

	contentRetriever := content.NewContentRetriever(mockRetriever)
	_, err := contentRetriever.RetrieveContent(manifest.ID)

	if err == nil {
		t.Fatal("Expected an error due to overall content integrity mismatch, but got nil")
	}
	expectedErrorMsg := "overall content integrity check failed"
	if !strings.Contains(err.Error(), expectedErrorMsg) {
		t.Errorf("Expected error message to contain '%s', got '%v'", expectedErrorMsg, err)
	}
}

func TestRetrieveContent_SizeMismatch(t *testing.T) {
	originalContent := []byte("Size mismatch test content.")
	testChunker := chunking.NewBasicChunker(10)
	chunks, _ := testChunker.ChunkContent(originalContent)
	manifest, _ := testChunker.GenerateManifest(chunks, originalContent)

	manifest.TotalSize = manifest.TotalSize + 10

	mockRetriever := retriever.NewMockRetriever()
	mockRetriever.AddManifest(manifest)
	for _, ch := range chunks {
		mockRetriever.AddChunk(ch)
	}

	contentRetriever := content.NewContentRetriever(mockRetriever)
	_, err := contentRetriever.RetrieveContent(manifest.ID)

	if err == nil {
		t.Fatal("Expected an error due to content size mismatch, but got nil")
	}
	expectedErrorMsg := "reassembled content size mismatch"
	if !strings.Contains(err.Error(), expectedErrorMsg) {
		t.Errorf("Expected error message to contain '%s', got '%v'", expectedErrorMsg, err)
	}
}

func TestRetrieveContent_EmptyContent(t *testing.T) {
	emptyContentHash := hashTestData([]byte{})
	manifest := &chunking.Manifest{
		ID:        hashTestData([]byte("empty_manifest_id_data")),
		ContentID: emptyContentHash,
		ChunkIDs:  []string{},
		TotalSize: 0,
	}

	mockRetriever := retriever.NewMockRetriever()
	mockRetriever.AddManifest(manifest)

	contentRetriever := content.NewContentRetriever(mockRetriever)
	retrievedData, err := contentRetriever.RetrieveContent(manifest.ID)
	if err != nil {
		t.Fatalf("RetrieveContent for empty content failed unexpectedly: %v", err)
	}

	if len(retrievedData) != 0 {
		t.Errorf("Retrieved content for an empty manifest should be empty. Got %d bytes.", len(retrievedData))
	}
}

func TestRetrieveContent_ManifestWithNoChunksButNonZeroSize(t *testing.T) {
	manifest := &chunking.Manifest{
		ID:        "no_chunks_bad_size_manifest",
		ContentID: "some_content_id",
		ChunkIDs:  []string{},
		TotalSize: 100,
	}

	mockRetriever := retriever.NewMockRetriever()
	mockRetriever.AddManifest(manifest)

	contentRetriever := content.NewContentRetriever(mockRetriever)
	_, err := contentRetriever.RetrieveContent(manifest.ID)

	if err == nil {
		t.Fatal("Expected an error for manifest with no chunks but non-zero size, got nil")
	}
	expectedErrorMsg := "manifest no_chunks_bad_size_manifest contains no chunk IDs but has non-zero total size"
	if !strings.Contains(err.Error(), expectedErrorMsg) {
		t.Errorf("Expected error message '%s', got '%v'", expectedErrorMsg, err)
	}
}

func TestRetrieveContent_FetchManifestError(t *testing.T) {
    mockRetriever := retriever.NewMockRetriever()
    mockRetriever.FetchManifestFunc = func(manifestCID string) (*chunking.Manifest, error) {
        return nil, errors.New("simulated FetchManifest error")
    }
    contentRetriever := content.NewContentRetriever(mockRetriever)

    _, err := contentRetriever.RetrieveContent("any_cid")
    if err == nil {
        t.Fatal("Expected an error from FetchManifest, got nil")
    }
    if !strings.Contains(err.Error(), "simulated FetchManifest error") {
        t.Errorf("Expected error message to contain 'simulated FetchManifest error', got '%v'", err)
    }
}

func TestRetrieveContent_FetchChunkError(t *testing.T) {
    manifest := &chunking.Manifest{
        ID:        "manifest_with_one_chunk",
        ContentID: "content_id_for_one_chunk",
        ChunkIDs:  []string{"chunk1_cid"},
        TotalSize: 10,
    }
    mockRetriever := retriever.NewMockRetriever()
    mockRetriever.AddManifest(manifest)
    mockRetriever.FetchChunkFunc = func(chunkCID string) (chunking.Chunk, error) {
        if chunkCID == "chunk1_cid" {
            return chunking.Chunk{}, errors.New("simulated FetchChunk error")
        }
        return chunking.Chunk{}, fmt.Errorf("unexpected chunk fetch attempt: %s", chunkCID)
    }

    contentRetriever := content.NewContentRetriever(mockRetriever)
    _, err := contentRetriever.RetrieveContent(manifest.ID)

    if err == nil {
        t.Fatal("Expected an error from FetchChunk, got nil")
    }
    if !strings.Contains(err.Error(), "simulated FetchChunk error") {
        t.Errorf("Expected error message to contain 'simulated FetchChunk error', got '%v'", err)
    }
}
