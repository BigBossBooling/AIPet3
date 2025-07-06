// pkg/dds/service/service.go
package service

import (
	"bytes"
	"crypto/sha256"
	"digisocialblock/pkg/dds/chunking"
	"digisocialblock/pkg/dds/network" // Added for P2P service and discovery
	"digisocialblock/pkg/dds/originator"
	"digisocialblock/pkg/dds/retriever" // Added in Task 2.2
	"digisocialblock/pkg/dds/storage"
	"encoding/hex"
	"fmt"
	// "sort" // Removed unused import
)

// DDSService defines the high-level interface for interacting with the DDS.
type DDSService interface {
	Publish(content []byte) (string, error) // Returns manifest CID
	Retrieve(manifestCID string) ([]byte, error)
}

// DDSCoreService is the main implementation of the DDSService.
type DDSCoreService struct {
	chunker      chunking.Chunker
	storage      storage.Storage // Local storage
	originator   originator.Originator
	retriever    retriever.Retriever       // For direct retrieval logic (Task 2.2)
	p2pService   network.P2PService        // For network interactions (Task 2.3)
	peerDiscover network.PeerDiscovery     // For finding peers (Task 2.3)
}

// NewDDSCoreService creates a new DDSCoreService.
func NewDDSCoreService(
	c chunking.Chunker,
	s storage.Storage,
	o originator.Originator,
	r retriever.Retriever, // Added in Task 2.2
	p2p network.P2PService, // Added in Task 2.3
	discover network.PeerDiscovery, // Added in Task 2.3
) *DDSCoreService {
	return &DDSCoreService{
		chunker:      c,
		storage:      s,
		originator:   o,
		retriever:    r,
		p2pService:   p2p,
		peerDiscover: discover,
	}
}

// hashData utility (should be consistent with chunking and retriever)
func hashData(data []byte) string {
	hash := sha256.Sum256(data)
	return hex.EncodeToString(hash[:])
}

// Publish processes content, stores it locally, and advertises it to the network.
func (s *DDSCoreService) Publish(content []byte) (string, error) {
	chunks, err := s.chunker.ChunkContent(content)
	if err != nil {
		return "", fmt.Errorf("dds service publish: failed to chunk content: %w", err)
	}

	manifest, err := s.chunker.GenerateManifest(chunks, content)
	if err != nil {
		return "", fmt.Errorf("dds service publish: failed to generate manifest: %w", err)
	}

	// Store locally first
	for _, chunk := range chunks {
		if err := s.storage.StoreChunk(chunk); err != nil {
			return "", fmt.Errorf("dds service publish: failed to store chunk %s locally: %w", chunk.ID, err)
		}
	}
	if err := s.storage.StoreManifest(manifest); err != nil {
		return "", fmt.Errorf("dds service publish: failed to store manifest %s locally: %w", manifest.ID, err)
	}

	// Advertise to originator (e.g., local indexing, initial seed)
	if err := s.originator.AdvertiseContent(manifest.ID); err != nil {
		fmt.Printf("dds service publish: warning - failed to advertise content %s to originator: %v\n", manifest.ID, err)
	}

	// Advertise to P2P network
	if s.p2pService != nil {
		if err := s.p2pService.AdvertiseContent(manifest.ID); err != nil {
			// This might be a non-fatal error for the publish operation itself,
			// as content is already stored locally.
			fmt.Printf("dds service publish: warning - failed to advertise content %s to P2P network: %v\n", manifest.ID, err)
		}
	} else {
		fmt.Println("dds service publish: P2P service not configured, skipping network advertisement.")
	}


	return manifest.ID, nil
}

// Retrieve content. It first tries local storage, then falls back to the P2P network.
func (s *DDSCoreService) Retrieve(manifestCID string) ([]byte, error) {
	if manifestCID == "" {
		return nil, fmt.Errorf("manifest CID cannot be empty for retrieval")
	}

	// 1. Try fetching from local storage first (via the retriever logic if it uses local storage)
	// Assuming s.retriever is configured to use s.storage or a similar mechanism for local checks.
	// The ContentRetriever created in Task 2.2 uses its retriever dependency.
	// If that retriever is a MockRetriever seeded from local storage, it works.
	// Let's refine this to be more explicit: try local storage directly, then network.

	// Attempt to get manifest from local storage
	manifest, err := s.storage.GetManifest(manifestCID)
	if err == nil && manifest != nil {
		// Manifest found locally, try to assemble from local chunks
		var assembledContent bytes.Buffer
		chunkDataMap := make(map[string][]byte, len(manifest.ChunkIDs))

		for _, chunkCID := range manifest.ChunkIDs {
			chunk, chunkErr := s.storage.GetChunk(chunkCID)
			if chunkErr != nil {
				// Local chunk missing, break and try network
				fmt.Printf("dds service retrieve: chunk %s for manifest %s not found locally, attempting network retrieval\n", chunkCID, manifestCID)
				manifest = nil // Signal that we need to fetch from network
				break
			}
			recalculatedChunkID := hashData(chunk.Data)
			if chunk.ID != recalculatedChunkID {
				return nil, fmt.Errorf("local chunk %s integrity failed: expected %s, got %s", chunk.ID, chunk.ID, recalculatedChunkID)
			}
			chunkDataMap[chunk.ID] = chunk.Data
		}

		if manifest != nil { // If all chunks were found locally
			for _, chunkCID := range manifest.ChunkIDs {
				assembledContent.Write(chunkDataMap[chunkCID])
			}
			finalContent := assembledContent.Bytes()
			if int64(len(finalContent)) != manifest.TotalSize {
				return nil, fmt.Errorf("local reassembled content size mismatch for %s", manifestCID)
			}
			recalculatedContentID := hashData(finalContent)
			if manifest.ContentID != recalculatedContentID {
				return nil, fmt.Errorf("local overall content integrity check failed for %s", manifestCID)
			}
			fmt.Printf("dds service retrieve: content for manifest %s successfully retrieved from local storage\n", manifestCID)
			return finalContent, nil
		}
	}
	// If manifest not found locally, or local chunks were missing, try P2P network
	fmt.Printf("dds service retrieve: manifest %s not found locally or incomplete, attempting P2P network retrieval\n", manifestCID)
	if s.p2pService == nil || s.peerDiscover == nil {
		return nil, fmt.Errorf("dds service retrieve: P2P service or peer discovery not configured, cannot fetch from network")
	}

	peers, err := s.peerDiscover.DiscoverPeers()
	if err != nil {
		return nil, fmt.Errorf("dds service retrieve: failed to discover peers: %w", err)
	}
	if len(peers) == 0 {
		return nil, fmt.Errorf("dds service retrieve: no peers found to request content %s", manifestCID)
	}

	// Try fetching from discovered peers
	// Simple strategy: try first few peers or peers with good reputation (conceptual)
	// For this mock, we might just try the first one that claims to have it.
	var fetchedManifest *chunking.Manifest
	var peerThatHasManifest network.Node

	for _, peer := range peers {
		// In a real system, we might check if peer advertises this manifestID
		// For mock, MockP2PService.RequestManifest might check peer.KnownContent
		fmt.Printf("dds service retrieve: requesting manifest %s from peer %s\n", manifestCID, peer.ID)
		m, fetchErr := s.p2pService.RequestManifest(peer, manifestCID)
		if fetchErr == nil && m != nil {
			fetchedManifest = m
			peerThatHasManifest = peer
			fmt.Printf("dds service retrieve: received manifest %s from peer %s\n", manifestCID, peer.ID)
			break
		}
		fmt.Printf("dds service retrieve: peer %s did not provide manifest %s (error: %v)\n", peer.ID, manifestCID, fetchErr)
	}

	if fetchedManifest == nil {
		return nil, fmt.Errorf("dds service retrieve: could not fetch manifest %s from any discovered peer", manifestCID)
	}

	// Now fetch chunks for the fetchedManifest from the peerThatHasManifest
	var assembledContentNet bytes.Buffer
	retrievedChunksNet := make(map[string]chunking.Chunk)
    chunkDataMapNet := make(map[string][]byte, len(fetchedManifest.ChunkIDs))

	for _, chunkCID := range fetchedManifest.ChunkIDs {
		fmt.Printf("dds service retrieve: requesting chunk %s from peer %s\n", chunkCID, peerThatHasManifest.ID)
		chunk, chunkErr := s.p2pService.RequestChunk(peerThatHasManifest, chunkCID)
		if chunkErr != nil {
			return nil, fmt.Errorf("dds service retrieve: failed to fetch chunk %s from peer %s: %w", chunkCID, peerThatHasManifest.ID, chunkErr)
		}
		recalculatedChunkID := hashData(chunk.Data)
		if chunk.ID != recalculatedChunkID {
			return nil, fmt.Errorf("network chunk %s integrity failed (from peer %s): expected %s, got %s", chunk.ID, peerThatHasManifest.ID, chunk.ID, recalculatedChunkID)
		}
		retrievedChunksNet[chunk.ID] = chunk
        chunkDataMapNet[chunk.ID] = chunk.Data
	}

	for _, chunkCID := range fetchedManifest.ChunkIDs {
        data, ok := chunkDataMapNet[chunkCID]
        if !ok {
            return nil, fmt.Errorf("internal error: network chunk %s data not found after fetching for manifest %s", chunkCID, manifestCID)
        }
        assembledContentNet.Write(data)
    }

	finalContentNet := assembledContentNet.Bytes()

	if int64(len(finalContentNet)) != fetchedManifest.TotalSize {
		return nil, fmt.Errorf("network reassembled content size mismatch for %s (from peer %s)", manifestCID, peerThatHasManifest.ID)
	}
	recalculatedContentIDNet := hashData(finalContentNet)
	if fetchedManifest.ContentID != recalculatedContentIDNet {
		return nil, fmt.Errorf("network overall content integrity check failed for %s (from peer %s)", manifestCID, peerThatHasManifest.ID)
	}

	// Optionally, store the fetched content locally after successful retrieval and verification
	fmt.Printf("dds service retrieve: content for manifest %s successfully retrieved from network peer %s. Storing locally.\n", manifestCID, peerThatHasManifest.ID)
	for _, chunk := range retrievedChunksNet {
		if err := s.storage.StoreChunk(chunk); err != nil {
			fmt.Printf("dds service retrieve: warning - failed to store network chunk %s locally: %v\n", chunk.ID, err)
		}
	}
	if err := s.storage.StoreManifest(fetchedManifest); err != nil {
		fmt.Printf("dds service retrieve: warning - failed to store network manifest %s locally: %v\n", fetchedManifest.ID, err)
	}


	return finalContentNet, nil
}
