// cmd/test_user_profiles/main.go
package main

import (
	"digisocialblock/core/content"
	"digisocialblock/core/user"
	"digisocialblock/pkg/dds/chunking"
	"digisocialblock/pkg/dds/network"
	"digisocialblock/pkg/dds/originator"
	"digisocialblock/pkg/dds/retriever"
	"digisocialblock/pkg/dds/storage"
	// "digisocialblock/pkg/dds/service" // Not directly using DDSCoreService here, but its components
	"digisocialblock/pkg/identity"
	"fmt"
	"log"
)

func main() {
	fmt.Println("--- Digisocialblock User Profile DDS Integration Test ---")

	// 1. Setup User Identity (Wallet)
	fmt.Println("\n--- Setting up User Wallet ---")
	testUserWallet, err := identity.NewWallet()
	if err != nil {
		log.Fatalf("Failed to create user wallet: %v", err)
	}
	fmt.Printf("User Wallet created. Address: %s\n", testUserWallet.GetAddress())

	// 2. Setup DDS Components (Mocks)
	// These would be shared or accessible by the services
	fmt.Println("\n--- Setting up Mock DDS Components ---")
	mockChunker := chunking.NewBasicChunker(1024) // 1KB chunk size
	mockLocalStorage := storage.NewInMemoryStorage()
	mockOriginator := originator.NewMockOriginator()

	// Setup for ContentPublisher
	contentPublisher := content.NewContentPublisher(mockChunker, mockLocalStorage, mockOriginator)
	fmt.Println("Mock ContentPublisher initialized.")

	// Setup for ContentRetriever
	// The MockRetriever will be seeded with data from mockLocalStorage by the test logic
	mockRetriever := retriever.NewMockRetriever()
	// Seed function for MockRetriever based on what publisher stores in mockLocalStorage
	seedRetriever := func(manifestCID string) error {
		m, err := mockLocalStorage.GetManifest(manifestCID)
		if err != nil {
			return fmt.Errorf("seeding retriever: failed to get manifest %s: %w", manifestCID, err)
		}
		mockRetriever.AddManifest(m)
		for _, chunkID := range m.ChunkIDs {
			c, err := mockLocalStorage.GetChunk(chunkID)
			if err != nil {
				return fmt.Errorf("seeding retriever: failed to get chunk %s: %w", chunkID, err)
			}
			mockRetriever.AddChunk(c)
		}
		return nil
	}
	contentRetriever := content.NewContentRetriever(mockRetriever)
	fmt.Println("Mock ContentRetriever initialized.")


	// 3. Initialize ProfileManager
	profileManager := user.NewProfileManager(contentPublisher, contentRetriever)
	fmt.Println("ProfileManager initialized.")

	// 4. Create a new Profile
	fmt.Println("\n--- Creating and Publishing Initial Profile ---")
	initialProfile, err := user.NewProfile(
		testUserWallet.GetAddress(),
		"Test User One",
		"This is a bio for the first test user of Digisocialblock!",
		"", // No profile picture CID initially
	)
	if err != nil {
		log.Fatalf("Failed to create initial profile struct: %v", err)
	}
	fmt.Printf("Initial profile struct created for %s (Version %d)\n", initialProfile.DisplayName, initialProfile.Version)

	// 5. Publish the initial Profile using ProfileManager
	profileCIDv1, err := profileManager.PublishProfile(initialProfile)
	if err != nil {
		log.Fatalf("Failed to publish initial profile: %v", err)
	}
	fmt.Printf("Initial profile published. CID: %s\n", profileCIDv1)

	// 6. Retrieve the published profile to verify
	fmt.Println("\n--- Retrieving Initial Profile for Verification ---")
	// Seed the retriever with the data publisher just stored
	if err := seedRetriever(profileCIDv1); err != nil {
		log.Fatalf("Failed to seed retriever for v1: %v", err)
	}

	retrievedProfileV1, err := profileManager.RetrieveProfile(profileCIDv1)
	if err != nil {
		log.Fatalf("Failed to retrieve profile v1 (CID: %s): %v", profileCIDv1, err)
	}

	// Verification
	if retrievedProfileV1.DisplayName != initialProfile.DisplayName ||
		retrievedProfileV1.Bio != initialProfile.Bio ||
		retrievedProfileV1.OwnerAddress != initialProfile.OwnerAddress ||
		retrievedProfileV1.Version != initialProfile.Version {
		log.Fatalf("Retrieved profile v1 does not match original!\nOriginal: %+v\nRetrieved: %+v", initialProfile, retrievedProfileV1)
	}
	fmt.Println("SUCCESS: Retrieved profile v1 matches original published profile.")

	// 7. Update the Profile
	fmt.Println("\n--- Updating and Publishing Profile (Version 2) ---")
	updatedDisplayName := "Test User One (Updated)"
	updatedBio := "Bio has been updated. Now with more decentralization!"
	// Simulate getting a CID for a new profile picture
	newProfilePicCID := "bafybeigdyrzt5sfp7udm7hu76uh7y26nf3efuylqabf3oclgtqy55fbzdi" // Example IPFS CID

	// Use the retrievedProfileV1 (or initialProfile, as it's the same content) to update
	// It's important to use the struct that reflects the current version we want to update
	profileCIDv2, updatedProfileV2, err := profileManager.UpdateAndPublishProfile(
		retrievedProfileV1, // or initialProfile, since it's v1
		updatedDisplayName,
		updatedBio,
		newProfilePicCID,
	)
	if err != nil {
		log.Fatalf("Failed to update and publish profile v2: %v", err)
	}
	if updatedProfileV2.Version != 2 {
		log.Fatalf("Expected profile version to be 2 after update, got %d", updatedProfileV2.Version)
	}
	fmt.Printf("Profile updated and published as v%d. New CID: %s\n", updatedProfileV2.Version, profileCIDv2)
	fmt.Printf("Updated Profile Struct: %+v\n", updatedProfileV2)


	// 8. Retrieve the updated profile (v2) to verify
	fmt.Println("\n--- Retrieving Updated Profile (Version 2) for Verification ---")
	// Seed the retriever with the new v2 data
	if err := seedRetriever(profileCIDv2); err != nil {
		log.Fatalf("Failed to seed retriever for v2: %v", err)
	}

	retrievedProfileV2, err := profileManager.RetrieveProfile(profileCIDv2)
	if err != nil {
		log.Fatalf("Failed to retrieve profile v2 (CID: %s): %v", profileCIDv2, err)
	}

	// Verification for v2
	if retrievedProfileV2.DisplayName != updatedDisplayName ||
		retrievedProfileV2.Bio != updatedBio ||
		retrievedProfileV2.ProfilePictureCID != newProfilePicCID ||
		retrievedProfileV2.OwnerAddress != initialProfile.OwnerAddress || // Owner address should not change
		retrievedProfileV2.Version != 2 {
		log.Fatalf("Retrieved profile v2 does not match expected updated profile!\nExpected: DisplayName='%s', Bio='%s', PicCID='%s', Version=2\nGot: %+v",
			updatedDisplayName, updatedBio, newProfilePicCID, retrievedProfileV2)
	}
	fmt.Println("SUCCESS: Retrieved profile v2 matches the updated published profile.")


	// --- Simulate another user trying to retrieve Node 1's V2 Profile via P2P (Conceptual) ---
	// This part requires DDSCoreService and P2P mocks from Task 2.3

	fmt.Println("\n--- Simulating Node 2 Retrieving User 1's Profile (v2) via P2P ---")

	// Setup Node 1's full DDS Service (as if it published Profile v2)
	node1Chunker := chunking.NewBasicChunker(1024)
	node1LocalStorage := storage.NewInMemoryStorage() // Fresh storage for this part of test
	// node1Originator := originator.NewMockOriginator() // Unused
	// node1Retriever := retriever.NewMockRetriever() // Node 1's own retriever - Unused
	node1P2PIdentity, _ := network.NewNode("localhost:7001", 100)
	node1P2PService := network.NewMockP2PService(node1P2PIdentity)
	// node1Discovery := network.NewMockPeerDiscovery() // Unused

	// Manually publish profile v2 data to node1's local storage and advertise it
	// This is to ensure node1P2PService has the data for node2 to fetch
	profileV2Data, _ := updatedProfileV2.ToJSON()
	v2Chunks, _ := node1Chunker.ChunkContent(profileV2Data)
	v2Manifest, _ := node1Chunker.GenerateManifest(v2Chunks, profileV2Data)
	// Crucially, ensure this manifest ID matches profileCIDv2 if hashing is deterministic
    // If not, the test needs to use v2Manifest.ID for retrieval.
    // For simplicity, let's assume profileCIDv2 IS v2Manifest.ID due to deterministic hashing.
    // If there's any non-determinism (e.g. timestamp in manifest ID generation), this test will fail.
    // The ContentPublisher already returns the manifest ID, so profileCIDv2 is the one to use.

	for _, chunk := range v2Chunks {
		node1LocalStorage.StoreChunk(chunk)
	}
	node1LocalStorage.StoreManifest(v2Manifest)
	node1P2PService.LocalNode.AddAdvertisedContent(profileCIDv2) // Node 1 advertises it

	fmt.Printf("Node 1 (%s) now holds and advertises profile CID: %s\n", node1P2PIdentity.ID[:8], profileCIDv2)


	// Setup Node 2's DDS Service (retriever node)
	node2Chunker := chunking.NewBasicChunker(1024)
	node2LocalStorage := storage.NewInMemoryStorage() // Node 2 has empty local storage
	node2Originator := originator.NewMockOriginator()
	// node2Retriever := retriever.NewMockRetriever()  // Unused
	node2P2PIdentity, _ := network.NewNode("localhost:7002", 90)
	node2P2PService := network.NewMockP2PService(node2P2PIdentity)
	// node2Discovery := network.NewMockPeerDiscovery() // Unused // Will "discover" Node 1 by default

	// Configure Node 2's P2P service to be able to "request" from Node 1
	node2P2PService.AddPeerToNetworkView(node1P2PIdentity) // Node 2 knows about Node 1
	node2P2PService.RequestHandlerFunc = func(peer network.Node, requestType string, id string) (interface{}, error) {
		if peer.ID == node1P2PIdentity.ID {
			// Simulate Node 1 responding
			if requestType == "manifest" {
				fmt.Printf("  P2P SIM: Node 2 requesting MANIFEST %s from Node 1\n", id)
				return node1LocalStorage.GetManifest(id)
			}
			if requestType == "chunk" {
				fmt.Printf("  P2P SIM: Node 2 requesting CHUNK %s from Node 1\n", id)
				return node1LocalStorage.GetChunk(id)
			}
		}
		return nil, fmt.Errorf("P2P SIM: unhandled request to peer %s for %s %s", peer.ID, requestType, id)
	}

	// Node 2's ProfileManager (using Node 2's DDS components)
	// For this P2P test, ContentPublisher for Node 2 is not strictly needed, only retriever part.
	// But ProfileManager needs both. So we provide Node2's components.
	node2ProfileManager := user.NewProfileManager(
		content.NewContentPublisher(node2Chunker, node2LocalStorage, node2Originator), // Node2's publisher
		content.NewContentRetriever(node2P2PService), // Node2's retriever uses its P2P service
	)
	fmt.Printf("Node 2 (%s) ProfileManager initialized for P2P retrieval.\n", node2P2PIdentity.ID[:8])

	fmt.Printf("Node 2 attempting to retrieve profile CID %s via P2P...\n", profileCIDv2)
	retrievedProfileV2_Node2, err := node2ProfileManager.RetrieveProfile(profileCIDv2)
	if err != nil {
		log.Fatalf("Node 2 failed to retrieve profile v2 via P2P (CID: %s): %v", profileCIDv2, err)
	}

	if retrievedProfileV2_Node2.DisplayName != updatedDisplayName ||
		retrievedProfileV2_Node2.Bio != updatedBio ||
		retrievedProfileV2_Node2.ProfilePictureCID != newProfilePicCID ||
		retrievedProfileV2_Node2.Version != 2 {
		log.Fatalf("Node 2 retrieved profile v2 via P2P does not match expected!\nExpected: DisplayName='%s', Bio='%s', PicCID='%s', Version=2\nGot: %+v",
			updatedDisplayName, updatedBio, newProfilePicCID, retrievedProfileV2_Node2)
	}
	fmt.Println("SUCCESS: Node 2 retrieved profile v2 via P2P and it matches the updated published profile.")


	fmt.Println("\n--- User Profile DDS Integration Test Complete ---")
}
