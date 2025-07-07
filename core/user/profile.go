// core/user/profile.go
package user

import (
	"encoding/json"
	"fmt"
	"time"
)

// Profile represents a user's profile data.
type Profile struct {
	OwnerAddress      string `json:"ownerAddress"`      // Wallet address of the profile owner (for association)
	DisplayName       string `json:"displayName"`       // User's chosen display name
	Bio               string `json:"bio,omitempty"`     // Optional user biography
	ProfilePictureCID string `json:"profilePictureCID,omitempty"` // Optional CID of an image stored on DDS
	Timestamp         int64  `json:"timestamp"`         // Unix nano timestamp of the last update
	Version           int    `json:"version"`           // Version number for the profile, incremented on each update
	// CustomFields map[string]string `json:"customFields,omitempty"` // For future extensibility
}

// NewProfile creates a new Profile instance.
func NewProfile(ownerAddress, displayName, bio, profilePictureCID string) (*Profile, error) {
	if ownerAddress == "" {
		return nil, fmt.Errorf("owner address cannot be empty")
	}
	if displayName == "" {
		return nil, fmt.Errorf("display name cannot be empty")
	}
	// Basic validation for display name length (example)
	if len(displayName) > 50 {
		return nil, fmt.Errorf("display name cannot exceed 50 characters")
	}
	if len(bio) > 500 { // Example limit for bio
		return nil, fmt.Errorf("bio cannot exceed 500 characters")
	}


	return &Profile{
		OwnerAddress:      ownerAddress,
		DisplayName:       displayName,
		Bio:               bio,
		ProfilePictureCID: profilePictureCID,
		Timestamp:         time.Now().UnixNano(),
		Version:           1, // Initial version
	}, nil
}

// Update modifies the profile with new data and increments the version.
// Only non-empty fields in 'updateData' will be used for update.
func (p *Profile) Update(displayName, bio, profilePictureCID string) error {
	if p == nil {
		return fmt.Errorf("cannot update a nil profile")
	}

	updated := false
	if displayName != "" && p.DisplayName != displayName {
		if len(displayName) > 50 {
			return fmt.Errorf("display name cannot exceed 50 characters")
		}
		p.DisplayName = displayName
		updated = true
	}
	if bio != "" && p.Bio != bio { // Allow setting bio to empty if it was not empty
		if len(bio) > 500 {
			return fmt.Errorf("bio cannot exceed 500 characters")
		}
		p.Bio = bio
		updated = true
	}
    // Allow explicitly clearing Bio or ProfilePictureCID by passing an empty string
    // if the current value is not already empty.
    if bio == "" && p.Bio != "" {
        p.Bio = ""
        updated = true
    }

	if profilePictureCID != "" && p.ProfilePictureCID != profilePictureCID {
		p.ProfilePictureCID = profilePictureCID
		updated = true
	}
    if profilePictureCID == "" && p.ProfilePictureCID != "" {
        p.ProfilePictureCID = ""
        updated = true
    }


	if updated {
		p.Version++
		p.Timestamp = time.Now().UnixNano()
	}
	return nil
}

// ToJSON serializes the Profile struct to a JSON byte slice.
func (p *Profile) ToJSON() ([]byte, error) {
	if p == nil {
		return nil, fmt.Errorf("cannot serialize nil profile")
	}
	return json.Marshal(p)
}

// FromJSON deserializes a JSON byte slice into a Profile struct.
func FromJSON(data []byte) (*Profile, error) {
	if len(data) == 0 {
		return nil, fmt.Errorf("cannot deserialize empty data into profile")
	}
	var p Profile
	err := json.Unmarshal(data, &p)
	if err != nil {
		return nil, fmt.Errorf("failed to unmarshal profile data: %w", err)
	}
	// Basic validation after unmarshal
	if p.OwnerAddress == "" {
		return nil, fmt.Errorf("deserialized profile missing owner address")
	}
	if p.DisplayName == "" {
		return nil, fmt.Errorf("deserialized profile missing display name")
	}
	if p.Version < 1 {
		return nil, fmt.Errorf("deserialized profile has invalid version: %d", p.Version)
	}
	return &p, nil
}
