// pkg/dds/originator/originator.go
package originator

import (
	// "fmt" // Example: if logging or specific errors needed
)

// Originator defines the interface for components responsible for
// advertising content to the DDS network.
type Originator interface {
	AdvertiseContent(manifestID string) error
	// Future methods: IsAdvertised(manifestID string) (bool, error), etc.
}

// MockOriginator provides a basic mock implementation for testing.
type MockOriginator struct {
	// Store advertised CIDs to verify calls if needed
	AdvertisedManifests map[string]bool
	SimulateError       bool // If true, AdvertiseContent will return an error
	ErrorToReturn       error // Specific error to return if SimulateError is true
}

// NewMockOriginator creates a new MockOriginator instance.
func NewMockOriginator() *MockOriginator {
	return &MockOriginator{
		AdvertisedManifests: make(map[string]bool),
	}
}

// AdvertiseContent simulates advertising the content by storing the manifestID.
func (mo *MockOriginator) AdvertiseContent(manifestID string) error {
	if mo.SimulateError {
		if mo.ErrorToReturn != nil {
			return mo.ErrorToReturn
		}
		return NewMockOriginatorError("mock originator advertise error")
	}
	mo.AdvertisedManifests[manifestID] = true
	// fmt.Printf("Originator: Successfully advertised manifest %s\n", manifestID)
	return nil
}

// WasAdvertised checks if a manifestID was "advertised" by this mock.
func (mo *MockOriginator) WasAdvertised(manifestID string) bool {
	_, found := mo.AdvertisedManifests[manifestID]
	return found
}

// ClearAdvertised allows resetting the mock's state for multiple tests.
func (mo *MockOriginator) ClearAdvertised() {
	mo.AdvertisedManifests = make(map[string]bool)
}


// MockOriginatorError is a custom error type for MockOriginator
type MockOriginatorError struct {
    message string
}

func NewMockOriginatorError(message string) *MockOriginatorError {
    return &MockOriginatorError{message: message}
}

func (e *MockOriginatorError) Error() string {
    return e.message
}
