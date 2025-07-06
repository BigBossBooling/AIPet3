// pkg/ledger/block.go
package ledger

import (
	"fmt"
	"time"
)

// NewBlock creates a new block in the blockchain.
// It takes the previous block's hash, index, and a list of transactions.
func NewBlock(index int64, previousHash string, transactions []Transaction) (*Block, error) {
	if index < 0 {
		return nil, fmt.Errorf("block index cannot be negative")
	}
	// previousHash can be empty for genesis block, validated by Blockchain.AddBlock

	block := &Block{
		Header: BlockHeader{
			Index:        index,
			Timestamp:    time.Now().UnixNano(),
			PreviousHash: previousHash,
			// MerkleRoot will be calculated based on transactions
		},
		Transactions: transactions,
	}

	merkleRoot, err := CalculateMerkleRoot(transactions)
	if err != nil {
		return nil, fmt.Errorf("failed to calculate merkle root for new block: %w", err)
	}
	block.Header.MerkleRoot = merkleRoot

	blockHash, err := block.CalculateBlockHash()
	if err != nil {
		return nil, fmt.Errorf("failed to calculate block hash for new block: %w", err)
	}
	block.Hash = blockHash

	return block, nil
}

// IsBlockValid checks the validity of a block with respect to a previous block.
// This includes checking index, previous hash, block hash, and merkle root.
func (b *Block) IsBlockValid(previousBlock *Block) (bool, error) {
	if b == nil {
		return false, fmt.Errorf("current block cannot be nil")
	}
	if previousBlock == nil { // This is the genesis block scenario
		if b.Header.Index != 0 {
			return false, fmt.Errorf("genesis block must have index 0, got %d", b.Header.Index)
		}
		if b.Header.PreviousHash != "" { // Genesis block's PreviousHash should be empty
			return false, fmt.Errorf("genesis block's PreviousHash must be empty, got %s", b.Header.PreviousHash)
		}
	} else { // Non-genesis block
		if previousBlock.Header.Index+1 != b.Header.Index {
			return false, fmt.Errorf("invalid block index: expected %d, got %d", previousBlock.Header.Index+1, b.Header.Index)
		}
		if previousBlock.Hash != b.Header.PreviousHash {
			return false, fmt.Errorf("invalid previous block hash: expected %s, got %s", previousBlock.Hash, b.Header.PreviousHash)
		}
	}

	// Recalculate hash to verify integrity
	recalculatedHash, err := b.CalculateBlockHash()
	if err != nil {
		return false, fmt.Errorf("failed to recalculate block hash: %w", err)
	}
	if b.Hash != recalculatedHash {
		return false, fmt.Errorf("invalid block hash: expected %s, got %s (recalculated)", recalculatedHash, b.Hash)
	}

	// Recalculate Merkle Root to verify transaction integrity
	recalculatedMerkleRoot, err := CalculateMerkleRoot(b.Transactions)
	if err != nil {
		return false, fmt.Errorf("failed to recalculate merkle root: %w", err)
	}
	if b.Header.MerkleRoot != recalculatedMerkleRoot {
		return false, fmt.Errorf("invalid merkle root: expected %s, got %s (recalculated)", recalculatedMerkleRoot, b.Header.MerkleRoot)
	}

	// Validate all transactions within the block
	for i, tx := range b.Transactions {
		if !tx.VerifySignature() { // Assuming VerifySignature uses tx.SenderPublicKey internally
			return false, fmt.Errorf("transaction %d in block %d has an invalid signature", i, b.Header.Index)
		}
	}

	return true, nil
}

// Note: CalculateBlockHash and CalculateMerkleRoot are in model.go
// Block and BlockHeader structs are also in model.go
// This file focuses on block creation and validation logic.
