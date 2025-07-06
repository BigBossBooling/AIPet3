// core/content/retriever.go
package content

import (
	"bytes"
	"crypto/sha256"
	"encoding/hex"
	"fmt"
	"sort"

	"digisocialblock/pkg/dds/chunking"
	"digisocialblock/pkg/dds/retriever"
	// "digisocialblock/pkg/dds/storage" // We need storage to get chunks if retriever only fetches manifest CIDs
)

// ContentRetriever orchestrates the retrieval and reassembly of content from DDS.
type ContentRetriever struct {
	retriever retriever.Retriever
	// storage   storage.Storage //  If retriever is only for manifests and higher-level fetching
}

// NewContentRetriever creates a new ContentRetriever instance.
// It takes interfaces for DDS components as dependencies.
// For now, assuming the retriever can fetch both manifests and chunks directly.
// If the retriever only fetches manifest CIDs and chunk CIDs, then storage would be needed here.
func NewContentRetriever(r retriever.Retriever /*, s storage.Storage */) *ContentRetriever {
	return &ContentRetriever{
		retriever: r,
		// storage:   s,
	}
}

// hashData generates a SHA256 hash for given data and returns its hex string.
// This should be consistent with the hashing used in chunking.
func hashData(data []byte) string {
	hash := sha256.Sum256(data)
	return hex.EncodeToString(hash[:])
}

// RetrieveContent fetches a manifest, its chunks, reassembles, and verifies the content.
func (cr *ContentRetriever) RetrieveContent(manifestCID string) ([]byte, error) {
	if manifestCID == "" {
		return nil, fmt.Errorf("manifest CID cannot be empty")
	}

	// 1. Fetch the manifest
	manifest, err := cr.retriever.FetchManifest(manifestCID)
	if err != nil {
		return nil, fmt.Errorf("failed to fetch manifest %s: %w", manifestCID, err)
	}
	if manifest == nil {
		return nil, fmt.Errorf("fetched manifest for CID %s is nil", manifestCID)
	}
	if len(manifest.ChunkIDs) == 0 {
		// Handle case of empty content or manifest with no chunks.
		// If TotalSize is 0, it's valid empty content.
		if manifest.TotalSize == 0 {
			// Verify ContentID against hash of empty data if applicable
			// For now, assume empty content has a specific, known ContentID or handle as per DDS spec.
			// Let's assume for now that empty content means empty byte slice and verify against its hash.
			expectedContentID := hashData([]byte{})
			if manifest.ContentID != expectedContentID && manifest.ContentID != "" { // Allow empty ContentID for truly empty manifest
				// This case might need more nuanced handling based on how empty content is defined by the DDS.
				// For now, if TotalSize is 0 but ContentID is non-empty and doesn't match hash of empty bytes, it's an issue.
				// However, if ContentID is also "", it might be a manifest for "no content".
			}
			return []byte{}, nil
		}
		return nil, fmt.Errorf("manifest %s contains no chunk IDs but has non-zero total size", manifestCID)
	}


	// 2. Fetch all chunks referenced in the manifest
	// For simplicity, fetching sequentially. In a real system, this could be parallelized.
	retrievedChunks := make(map[string]chunking.Chunk) // Map to store retrieved chunks by their ID
	var assembledContent bytes.Buffer // Use bytes.Buffer for efficient concatenation

	// Create a map to store chunk data by ID for reassembly
    chunkDataMap := make(map[string][]byte, len(manifest.ChunkIDs))

	for _, chunkCID := range manifest.ChunkIDs {
		chunk, err := cr.retriever.FetchChunk(chunkCID)
		if err != nil {
			return nil, fmt.Errorf("failed to fetch chunk %s for manifest %s: %w", chunkCID, manifestCID, err)
		}
		// Verify individual chunk integrity (optional, but good practice if chunk CIDs are hashes of chunk data)
		// This assumes chunk.ID is the hash of chunk.Data
		recalculatedChunkID := hashData(chunk.Data)
		if chunk.ID != recalculatedChunkID {
			return nil, fmt.Errorf("integrity check failed for chunk %s: expected CID %s, got %s from data", chunk.ID, chunk.ID, recalculatedChunkID)
		}
		retrievedChunks[chunk.ID] = chunk
        chunkDataMap[chunk.ID] = chunk.Data
	}

	// 3. Reassemble content in the correct order specified by manifest.ChunkIDs
	 for _, chunkCID := range manifest.ChunkIDs {
        data, ok := chunkDataMap[chunkCID]
        if !ok {
            // This should ideally not happen if all chunks were fetched successfully
            return nil, fmt.Errorf("internal error: chunk %s data not found after fetching for manifest %s", chunkCID, manifestCID)
        }
        assembledContent.Write(data)
    }


	finalContent := assembledContent.Bytes()

	// 4. Verify total size
	if int64(len(finalContent)) != manifest.TotalSize {
		return nil, fmt.Errorf("reassembled content size mismatch: expected %d, got %d for manifest %s", manifest.TotalSize, len(finalContent), manifestCID)
	}

	// 5. Verify overall content integrity by hashing reassembled content
	// and comparing with manifest.ContentID (which should be the hash of the original full content)
	recalculatedContentID := hashData(finalContent)
	if manifest.ContentID != recalculatedContentID {
		return nil, fmt.Errorf("overall content integrity check failed for manifest %s: expected ContentID %s, got %s from reassembled data", manifestCID, manifest.ContentID, recalculatedContentID)
	}

	// Sort chunk IDs from manifest and retrieved for consistent comparison (if needed, though direct reassembly handles order)
	manifestChunkIDsSorted := make([]string, len(manifest.ChunkIDs))
	copy(manifestChunkIDsSorted, manifest.ChunkIDs)
	sort.Strings(manifestChunkIDsSorted)

	retrievedChunkIDsSorted := make([]string, 0, len(retrievedChunks))
	for id := range retrievedChunks {
		retrievedChunkIDsSorted = append(retrievedChunkIDsSorted, id)
	}
	sort.Strings(retrievedChunkIDsSorted)

	// Verify all expected chunks were retrieved (already implicitly done by reassembly loop, but explicit check is fine)
	if len(manifest.ChunkIDs) != len(retrievedChunks) {
		// This check is somewhat redundant if the reassembly loop above completes,
		// as it would fail if a chunk ID from manifest.ChunkIDs was not found in retrievedChunks.
		// However, it's a good sanity check.
		return nil, fmt.Errorf("chunk count mismatch for manifest %s: expected %d, retrieved %d", manifestCID, len(manifest.ChunkIDs), len(retrievedChunks))
	}
	// Could also do a deep equality check on sorted chunk ID slices if necessary,
	// but successful reassembly and content hash verification are stronger proofs.

	return finalContent, nil
}
