// core/content/publisher.go
package content

import (
	"fmt"
	"digisocialblock/pkg/dds/chunking" // Assuming these modules exist or will be mocked
	"digisocialblock/pkg/dds/storage"
	"digisocialblock/pkg/dds/originator"
)

// ContentPublisher orchestrates the publishing of content to DDS.
type ContentPublisher struct {
	chunker   chunking.Chunker
	storage   storage.Storage
	originator originator.Originator
}

// NewContentPublisher creates a new ContentPublisher instance.
// It takes interfaces for DDS components as dependencies.
func NewContentPublisher(c chunking.Chunker, s storage.Storage, o originator.Originator) *ContentPublisher {
	return &ContentPublisher{
		chunker:   c,
		storage:   s,
		originator: o,
	}
}

// PublishContent processes raw content, chunks it, stores it, and simulates advertisement.
// It returns the Content ID (CID) of the published content's manifest.
func (cp *ContentPublisher) PublishContent(content []byte) (string, error) {
	// 1. Chunk the content
	chunks, err := cp.chunker.ChunkContent(content)
	if err != nil {
		return "", fmt.Errorf("failed to chunk content: %w", err)
	}

	// 2. Generate manifest
	manifest, err := cp.chunker.GenerateManifest(chunks, content)
	if err != nil {
		return "", fmt.Errorf("failed to generate manifest: %w", err)
	}

	// 3. Store chunks locally
	for _, chunk := range chunks {
		if err := cp.storage.StoreChunk(chunk); err != nil {
			return "", fmt.Errorf("failed to store chunk %s: %w", chunk.ID, err)
		}
	}

	// 4. Store manifest locally
	if err := cp.storage.StoreManifest(manifest); err != nil {
		return "", fmt.Errorf("failed to store manifest %s: %w", manifest.ID, err)
	}

	// 5. Simulate advertisement (Originator's role)
	// In a real DDS, this would involve broadcasting the manifest ID to peers.
	if err := cp.originator.AdvertiseContent(manifest.ID); err != nil {
		return "", fmt.Errorf("failed to advertise content %s: %w", manifest.ID, err)
	}

	return manifest.ID, nil // Return the CID of the manifest
}
