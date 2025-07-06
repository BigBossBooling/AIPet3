// pkg/ledger/model.go
package ledger

import (
	"crypto/sha256"
	"encoding/hex"
	"fmt" // Needed for fmt.Errorf and fmt.Sprintf
	"time"
	// "digisocialblock/pkg/identity" // Not needed directly in model.go for Transaction struct
)

// TransactionType defines the type of a transaction.
type TransactionType string

const (
	TxTypeGeneric     TransactionType = "GENERIC"
	TxTypePostCreated TransactionType = "POST_CREATED" // Example: User creates a new post
	TxTypeFollowUser  TransactionType = "FOLLOW_USER"  // Example: User follows another user
	// ... other transaction types as needed for Digisocialblock
)

// Transaction represents a single transaction in a block.
type Transaction struct {
	ID        string          `json:"id"`        // Unique transaction ID (e.g., hash of its content)
	Timestamp int64           `json:"timestamp"` // Unix timestamp in nanoseconds
	Type      TransactionType `json:"type"`      // Type of transaction
	// SenderPublicKey string       `json:"senderPublicKey"` // Hex-encoded public key of the sender
	// For Digisocialblock, we'll use the Wallet Address as the sender identifier
	// This simplifies things if we don't need direct pubkey crypto for validation outside signatures
	SenderAddress   string `json:"senderAddress"`
	Payload         []byte `json:"payload"`         // Transaction-specific data (e.g., post content CID, target user ID for follow)
	Signature       []byte `json:"signature"`       // ECDSA signature of (ID+Timestamp+Type+SenderAddress+Payload)
	SenderPublicKey []byte `json:"senderPublicKeyBytes"` // Added to store actual public key bytes for signature verification
}

// CalculateHash generates a SHA256 hash for the transaction's core content.
// This hash is what gets signed.
func (tx *Transaction) CalculateHash() ([]byte, error) {
	// Data to hash: Timestamp, Type, SenderAddress, Payload
	// ID and Signature are excluded as ID is derived from this hash (or other content)
	// and Signature is created from this hash.
	// SenderPublicKeyBytes is part of the identity, not the action's core content for hashing.
	// data := []byte(string(tx.Type) + tx.SenderAddress + string(tx.Payload)) // Unused variable
	// Include timestamp for uniqueness if not already part of ID generation logic
	// For simplicity here, let's assume ID generation will ensure uniqueness.
	// If ID itself is a hash of these fields + nonce, then that's fine.
	// Here, we'll hash a concatenation for the signature.

	// A more robust way:
	// var dataToHash []byte
	// dataToHash = append(dataToHash, []byte(fmt.Sprintf("%d", tx.Timestamp))...)
	// dataToHash = append(dataToHash, []byte(tx.Type)...)
	// dataToHssh = append(dataToHash, []byte(tx.SenderAddress)...)
	// dataToHash = append(dataToHash, tx.Payload...)

	// Simplified approach for now:
	// The exact fields and their order for hashing must be consistent between signing and verification.
	// Let's make it: Timestamp + Type + SenderAddress + Payload(string for consistency if it can vary)
	// For POST_CREATED, payload is CID (string). For FOLLOW_USER, payload might be target user address (string).
	// To ensure consistency, always convert payload to its string representation for hashing if it's not already.
	// However, for CID, it's already a string, so []byte(string(tx.Payload)) is fine.
	// For more complex binary payloads, a stable serialization (like protobuf or gob) would be better before hashing.

	// Let's refine the data to be hashed for signature:
	// Order: Timestamp (string) + Type (string) + SenderAddress (string) + Payload (hex string of bytes for consistency)
	// This ensures that even if payload is binary, its hex representation is consistently hashed.
	payloadHex := hex.EncodeToString(tx.Payload)
	dataToSignStr := time.Unix(0, tx.Timestamp).String() + string(tx.Type) + tx.SenderAddress + payloadHex

	hash := sha256.Sum256([]byte(dataToSignStr))
	return hash[:], nil
}


// NewTransaction creates a new transaction.
// The SenderPublicKeyBytes should be the actual public key bytes from the sender's wallet.
func NewTransaction(senderAddress string, txType TransactionType, payload []byte) (*Transaction, error) {
	if senderAddress == "" {
		return nil, fmt.Errorf("sender address cannot be empty")
	}
	// SenderPublicKeyBytes will be set during signing or explicitly provided if needed before.
	// For now, it's set to nil and expected to be populated before/during Sign.

	tx := &Transaction{
		Timestamp:     time.Now().UnixNano(),
		Type:          txType,
		SenderAddress: senderAddress,
		Payload:       payload,
		// ID will be set after hashing, Signature after signing
	}

	// Generate ID based on initial content (excluding signature itself)
	// A common way is to hash (Timestamp + Type + SenderAddress + Payload)
	// Let's use a simplified version of CalculateHash for ID generation
	// to ensure ID is stable before signing.
	idDataStr := time.Unix(0, tx.Timestamp).String() + string(tx.Type) + tx.SenderAddress + hex.EncodeToString(tx.Payload)
	idHash := sha256.Sum256([]byte(idDataStr))
	tx.ID = hex.EncodeToString(idHash[:])

	return tx, nil
}

// Note: Sign and VerifySignature methods for Transaction are now in transaction.go
// to keep cryptographic operations and identity package dependencies separate from the core model.


// BlockHeader defines the structure of a block's header.
type BlockHeader struct {
	Index        int64  `json:"index"`        // Position in the blockchain
	Timestamp    int64  `json:"timestamp"`    // Unix timestamp in nanoseconds
	PreviousHash string `json:"previousHash"` // Hash of the previous block
	MerkleRoot   string `json:"merkleRoot"`   // Merkle root of transactions in this block
	// Nonce for PoW, or validator signatures for PoS/PoA could be added here
}

// Block represents a block in the blockchain.
type Block struct {
	Header       BlockHeader   `json:"header"`
	Transactions []Transaction `json:"transactions"`
	Hash         string        `json:"hash"` // Hash of the block header
}

// CalculateBlockHash generates a SHA256 hash for the block's header.
func (b *Block) CalculateBlockHash() (string, error) {
	headerData := fmt.Sprintf("%d%d%s%s",
		b.Header.Index,
		b.Header.Timestamp,
		b.Header.PreviousHash,
		b.Header.MerkleRoot,
	)
	hash := sha256.Sum256([]byte(headerData))
	return hex.EncodeToString(hash[:]), nil
}

// TODO: Implement Merkle Tree calculation for transactions if needed for full verification.
// For now, MerkleRoot can be a hash of concatenated transaction IDs for simplicity.
func CalculateMerkleRoot(transactions []Transaction) (string, error) {
    if len(transactions) == 0 {
        return "", nil // Or a default hash for empty block
    }
    var txHashes []string
    for _, tx := range transactions {
        txHashes = append(txHashes, tx.ID) // Using tx.ID as it's already a hash
    }
    // Simple concatenation and hash for mock merkle root
    concatenatedHashes := ""
    for _, h := range txHashes {
        concatenatedHashes += h
    }
    hash := sha256.Sum256([]byte(concatenatedHashes))
    return hex.EncodeToString(hash[:]), nil
}
