// cmd/test_dds_ledger_integration/main.go
package main

import (
	"fmt"
	"log"
	"digisocialblock/pkg/ledger"
	"digisocialblock/pkg/identity"
	// "digisocialblock/core/content" // No longer directly used by main
	"digisocialblock/pkg/dds/chunking"
	"digisocialblock/pkg/dds/storage"
	"digisocialblock/pkg/dds/originator"
	"digisocialblock/pkg/dds/retriever"
	"digisocialblock/pkg/dds/network"  // Added for P2P
	"digisocialblock/pkg/dds/service"   // Added for DDSCoreService
)

func main() {
	fmt.Println("--- Digisocialblock DDS-Ledger Integration Test (P2P Scenario) ---")

	// Define samplePostContent at a higher scope
	samplePostContent := "This is my first decentralized post on Digisocialblock via P2P! It's going to be awesome."

	// --- Node 1 (Publisher) Setup ---
	fmt.Println("\n--- Setting up Node 1 (Publisher) ---")
	// 1. Initialize Blockchain (shared by all nodes in this simulation)
	blockchain := ledger.NewBlockchain()

	// 2. Generate a Wallet for User 1
	user1Wallet, err := identity.NewWallet()
	if err != nil {
		log.Fatalf("Node 1: Failed to create user wallet: %v", err)
	}
	fmt.Printf("Node 1: User Wallet created: %s\n", user1Wallet.GetAddress())

	// 3. Initialize DDS Components for Node 1
	node1Chunker := chunking.NewBasicChunker(10) // Chunk size 10 bytes
	node1LocalStorage := storage.NewInMemoryStorage()
	node1Originator := originator.NewMockOriginator()
	node1MockRetriever := retriever.NewMockRetriever() // Retriever for Node 1 (primarily for its own content)

	// P2P setup for Node 1
	node1P2PIdentity, _ := network.NewNode("localhost:8001", 100)
	node1P2PService := network.NewMockP2PService(node1P2PIdentity)
	node1Discovery := network.NewMockPeerDiscovery() // Node 1 will discover Node 2 later

	// DDSCoreService for Node 1
	node1DdsService := service.NewDDSCoreService(
		node1Chunker,
		node1LocalStorage,
		node1Originator,
		node1MockRetriever, // Node1's retriever can use its own local storage
		node1P2PService,
		node1Discovery,
	)
	fmt.Println("Node 1: DDS Service initialized.")

	fmt.Printf("\nNode 1: Sample post content: \"%s\"\n", samplePostContent)

	// 5. Node 1 Publishes content to DDS using DDSCoreService
	// This will store it in node1LocalStorage and advertise via node1P2PService
	contentCID, err := node1DdsService.Publish([]byte(samplePostContent))
	if err != nil {
		log.Fatalf("Node 1: Failed to publish content to DDS: %v", err)
	}
	fmt.Printf("Node 1: Content published to DDS. Manifest CID: %s\n", contentCID)
	// At this point, node1P2PService's LocalNode should have the contentCID in its KnownContent

	// 6. Node 1 Creates a PostCreated Transaction
	txPayload := []byte(contentCID) // Payload is the CID
	postTx, err := ledger.NewTransaction(user1Wallet.GetAddress(), ledger.TxTypePostCreated, txPayload)
	if err != nil {
		log.Fatalf("Node 1: Failed to create post transaction: %v", err)
	}

	// 7. Node 1 Signs the Transaction
	user1PrivKeyBytes, err := identity.PrivateKeyToBytes(user1Wallet.PrivateKey)
	if err != nil {
		log.Fatalf("Node 1: Failed to get private key bytes: %v", err)
	}
	// Set public key before signing (as per current Transaction.Sign logic)
	user1PubKeyBytes, _ := user1Wallet.GetPublicKeyBytes()
	postTx.SenderPublicKey = user1PubKeyBytes
	if err := postTx.Sign(user1PrivKeyBytes); err != nil { // Sign method in transaction.go now handles this
		log.Fatalf("Node 1: Failed to sign transaction: %v", err)
	}
	fmt.Printf("Node 1: Transaction created and signed (ID: %s)\n", postTx.ID)

	// 8. Node 1 Verifies its Transaction Signature
	if !postTx.VerifySignature() {
		log.Fatalf("Node 1: Transaction signature verification failed unexpectedly!")
	}
	fmt.Println("Node 1: Transaction signature verified successfully.")

	// 9. Node 1 Adds the Transaction to the Blockchain
	_, err = blockchain.AddBlock([]ledger.Transaction{*postTx}) // Dereference postTx
	if err != nil {
		log.Fatalf("Node 1: Failed to add block to blockchain: %v", err)
	}
	fmt.Println("Node 1: Transaction added to blockchain.")

	// 10. Verify the Blockchain's integrity
	if !blockchain.IsChainValid() {
		log.Fatalf("Blockchain is invalid after Node 1 added block!")
	}
	fmt.Println("Blockchain is valid.")

	// --- Node 2 (Retriever) Setup ---
	fmt.Println("\n--- Setting up Node 2 (Retriever) ---")
	user2Wallet, err := identity.NewWallet() // Wallet for a different user/node
	if err != nil {
		log.Fatalf("Node 2: Failed to create wallet: %v", err)
	}
	fmt.Printf("Node 2: User Wallet created: %s\n", user2Wallet.GetAddress())

	node2Chunker := chunking.NewBasicChunker(10)      // Node 2 has its own chunker (though not used for retrieve)
	node2LocalStorage := storage.NewInMemoryStorage() // Node 2 has EMPTY local storage initially
	node2Originator := originator.NewMockOriginator() // Node 2 has its own originator
	node2MockRetriever := retriever.NewMockRetriever() // Node 2's direct retriever

	// P2P setup for Node 2
	node2P2PIdentity, _ := network.NewNode("localhost:8002", 95)
	node2P2PService := network.NewMockP2PService(node2P2PIdentity)
	node2Discovery := network.NewMockPeerDiscovery()

	// IMPORTANT: For Node 2 to find Node 1 via P2P:
	// 1. Node 1's P2P service needs to be "aware" of its own content.
	//    (MockP2PService for Node 1 was initialized with node1P2PIdentity, which stores its advertised content)
	// 2. Node 2's PeerDiscovery needs to be able to "find" Node 1.
	//    (MockPeerDiscovery by default has localhost:8001 - node1P2PIdentity.Address)
	// 3. Node 2's MockP2PService needs to be able to "route" requests to Node 1's MockP2PService.
	//    This is the tricky part with mocks. We'll simulate this by making Node 2's P2P service
	//    aware of Node 1's P2P identity and its advertised content for this test.
	node2P2PService.AddPeerToNetworkView(node1P2PService.LocalNode) // Node 2 now "knows" about Node 1 and its content

	// DDSCoreService for Node 2
	node2DdsService := service.NewDDSCoreService(
		node2Chunker,
		node2LocalStorage, // Empty local storage
		node2Originator,
		node2MockRetriever, // This retriever is for *direct* non-P2P gets, will be empty for Node 2
		node2P2PService,    // This P2P service will be used for network fallback
		node2Discovery,
	)
	fmt.Println("Node 2: DDS Service initialized.")

	// --- Node 2 Retrieves Content ---
	fmt.Println("\n--- Node 2 Testing Content Retrieval from DDS via Ledger CID (expecting P2P fetch) ---")

	// 11. Node 2 Retrieves the transaction from the blockchain
	latestBlock := blockchain.GetLatestBlock() // Same shared blockchain
	if len(latestBlock.Transactions) == 0 {
		log.Fatalf("Node 2: No transactions found in the latest block to test retrieval.")
	}
	retrievedTx := latestBlock.Transactions[0]
	cidFromLedger := string(retrievedTx.Payload)
	fmt.Printf("Node 2: Retrieved CID '%s' from ledger. Attempting to fetch content...\n", cidFromLedger)

	// 12. Node 2 Retrieves content using its DDSCoreService
	// This should trigger a P2P lookup as node2LocalStorage is empty.
	retrievedPostContentBytes, err := node2DdsService.Retrieve(cidFromLedger)
	if err != nil {
		// If MockP2PService's RequestManifest/RequestChunk is not properly returning data based on
		// node1P2PService.LocalNode.KnownContent, this will fail.
		// The current MockP2PService returns dummy data if peer is known.
		// We need to make it return actual data from node1LocalStorage for a true E2E test.
		// This requires node2P2PService.RequestHandlerFunc to access node1LocalStorage.
		// For now, let's refine MockP2PService to use a shared storage for simulation.
		// OR, simpler: Seed node2's MockP2PService's peer's (Node1) KnownContent and have it return real chunks/manifest.
		// The current MockP2PService.RequestManifest/Chunk will return dummy data.
		// To make this test pass end-to-end with current mocks, we need to ensure that
		// when node2P2PService "requests" from node1P2PIdentity, it gets the *actual* manifest/chunks.
		// This requires node1P2PService to actually serve them. Our MockP2PService is too simple for that.

		// Let's adjust the test:
		// When node1 publishes, its MockP2PService (node1P2PService) has its LocalNode updated.
		// When node2's DDSCoreService calls node2P2PService.RequestManifest(node1P2PIdentity, cid),
		// node2P2PService needs to simulate fetching from node1P2PIdentity.
		// We can achieve this by having node2P2PService.RequestHandlerFunc access node1LocalStorage.

		// Simpler mock strategy for this test:
		// The node1P2PService.LocalNode (node1P2PIdentity) has the contentCID in KnownContent.
		// node2P2PService.NetworkView has node1P2PIdentity.
		// When node2P2PService.RequestManifest(node1P2PIdentity, contentCID) is called,
		// it should "get" the manifest from node1LocalStorage.
		// We can achieve this by making the `RequestHandlerFunc` of node2P2PService
		// check node1P2PIdentity and then fetch from node1LocalStorage.

		node2P2PService.RequestHandlerFunc = func(peer network.Node, requestType string, id string) (interface{}, error) {
			if peer.ID == node1P2PIdentity.ID { // Request is to Node 1
				if requestType == "manifest" {
					fmt.Printf("Node 2 (via P2P mock): Requesting MANIFEST %s from Node 1 (%s)\n", id, peer.ID)
					return node1LocalStorage.GetManifest(id)
				}
				if requestType == "chunk" {
					fmt.Printf("Node 2 (via P2P mock): Requesting CHUNK %s from Node 1 (%s)\n", id, peer.ID)
					return node1LocalStorage.GetChunk(id)
				}
			}
			return nil, fmt.Errorf("mock P2P: unhandled request for peer %s, type %s, id %s", peer.ID, requestType, id)
		}
		// Retry retrieval with the handler func set
		retrievedPostContentBytes, err = node2DdsService.Retrieve(cidFromLedger)
		if err != nil {
			log.Fatalf("Node 2: Failed to retrieve content from DDS via P2P: %v", err)
		}
	}

	retrievedPostContent := string(retrievedPostContentBytes)
	fmt.Printf("Node 2: Content retrieved from DDS via P2P: \"%s\"\n", retrievedPostContent)

	// 13. Node 2 Verifies retrieved content matches original
	if retrievedPostContent == samplePostContent {
		fmt.Println("Node 2: SUCCESS: Retrieved content via P2P matches original sample post content!")
	} else {
		fmt.Println("Node 2: ERROR: Retrieved content via P2P DOES NOT MATCH original sample post content.")
		log.Fatalf("Node 2 Mismatch: \nOriginal: %s\nRetrieved: %s", samplePostContent, retrievedPostContent)
	}


	fmt.Println("\n--- DDS-Ledger P2P Integration Test Complete ---")
}
