// pkg/ledger/blockchain.go
package ledger

import (
	"fmt"
	"sync"
)

// Blockchain represents the chain of blocks.
type Blockchain struct {
	Blocks []*Block
	mu     sync.RWMutex // For thread-safe access to the blockchain
	// Could add a difficulty field for PoW, or validator set for PoS/PoA
}

// NewBlockchain creates and initializes a new blockchain with a genesis block.
func NewBlockchain() *Blockchain {
	genesisBlock, err := NewBlock(0, "", []Transaction{}) // Index 0, no previous hash, no transactions
	if err != nil {
		// This should ideally not happen for a hardcoded genesis block.
		// If it does, it indicates a fundamental issue with NewBlock or its dependencies.
		panic(fmt.Sprintf("failed to create genesis block: %v", err))
	}

	return &Blockchain{
		Blocks: []*Block{genesisBlock},
	}
}

// AddBlock adds a new block to the blockchain after validating it.
// It also validates all transactions within the block.
func (bc *Blockchain) AddBlock(transactions []Transaction) (*Block, error) {
	bc.mu.Lock()
	defer bc.mu.Unlock()

	if len(bc.Blocks) == 0 {
		return nil, fmt.Errorf("blockchain not initialized with a genesis block")
	}
	previousBlock := bc.GetLatestBlock()

	// Validate transactions before creating a new block
	for i, tx := range transactions {
		// Ensure SenderPublicKey is set for signature verification
		if len(tx.SenderPublicKey) == 0 {
			return nil, fmt.Errorf("transaction %d (ID: %s) is missing SenderPublicKey, cannot verify", i, tx.ID)
		}
		if !tx.VerifySignature() {
			return nil, fmt.Errorf("transaction %d (ID: %s) has an invalid signature", i, tx.ID)
		}
	}

	newBlock, err := NewBlock(previousBlock.Header.Index+1, previousBlock.Hash, transactions)
	if err != nil {
		return nil, fmt.Errorf("failed to create new block: %w", err)
	}

	// Validate the new block against the previous one
	// (IsBlockValid also re-verifies transactions if implemented thoroughly)
	isValid, validationErr := newBlock.IsBlockValid(previousBlock)
	if !isValid {
		return nil, fmt.Errorf("new block is invalid: %w", validationErr)
	}

	bc.Blocks = append(bc.Blocks, newBlock)
	return newBlock, nil
}

// GetLatestBlock returns the most recent block in the chain.
func (bc *Blockchain) GetLatestBlock() *Block {
	bc.mu.RLock()
	defer bc.mu.RUnlock()
	if len(bc.Blocks) == 0 {
		return nil // Should not happen if NewBlockchain ensures genesis block
	}
	return bc.Blocks[len(bc.Blocks)-1]
}

// IsChainValid checks the integrity of the entire blockchain.
// It iterates through all blocks and verifies their hashes and links.
func (bc *Blockchain) IsChainValid() bool {
	bc.mu.RLock()
	defer bc.mu.RUnlock()

	if len(bc.Blocks) == 0 {
		// An empty chain could be considered invalid or simply uninitialized
		// Depending on application logic. For a chain with a genesis block,
		// length should always be >= 1.
		return false
	}

	// Check genesis block separately if it has special validation rules
	// (e.g. index 0, empty previous hash)
	genesisBlock := bc.Blocks[0]
	isValidGenesis, err := genesisBlock.IsBlockValid(nil) // Pass nil as previousBlock for genesis
	if !isValidGenesis {
		fmt.Printf("Chain validation failed: Genesis block is invalid: %v\n", err)
		return false
	}


	for i := 1; i < len(bc.Blocks); i++ {
		currentBlock := bc.Blocks[i]
		previousBlock := bc.Blocks[i-1]

		isValid, validationErr := currentBlock.IsBlockValid(previousBlock)
		if !isValid {
			fmt.Printf("Chain validation failed at block %d: %v\n", currentBlock.Header.Index, validationErr)
			return false
		}
	}
	return true
}

// GetBlockByIndex returns a block by its index.
func (bc *Blockchain) GetBlockByIndex(index int64) (*Block, error) {
    bc.mu.RLock()
    defer bc.mu.RUnlock()
    if index < 0 || index >= int64(len(bc.Blocks)) {
        return nil, fmt.Errorf("block index %d out of bounds", index)
    }
    return bc.Blocks[index], nil
}

// GetTransactionByID searches the entire blockchain for a transaction by its ID.
// Note: This is inefficient for large chains. A real system would use an index.
func (bc *Blockchain) GetTransactionByID(txID string) (*Transaction, error) {
    bc.mu.RLock()
    defer bc.mu.RUnlock()
    for _, block := range bc.Blocks {
        for _, tx := range block.Transactions {
            if tx.ID == txID {
                return &tx, nil
            }
        }
    }
    return nil, fmt.Errorf("transaction with ID %s not found", txID)
}
