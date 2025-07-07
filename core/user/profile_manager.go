// core/user/profile_manager.go
package user

import (
	"digisocialblock/core/content" // For ContentPublisher and ContentRetriever
	"fmt"
)

// ProfileManager handles the creation, updating, and retrieval of user profiles
// by interacting with the DDS content publishing and retrieval services.
type ProfileManager struct {
	publisher  *content.ContentPublisher  // From Task 2.1
	retriever  *content.ContentRetriever // From Task 2.2
}

// NewProfileManager creates a new ProfileManager instance.
func NewProfileManager(publisher *content.ContentPublisher, retriever *content.ContentRetriever) *ProfileManager {
	if publisher == nil {
		// Or handle this by returning an error, for robustness
		panic("ProfileManager: ContentPublisher cannot be nil")
	}
	if retriever == nil {
		panic("ProfileManager: ContentRetriever cannot be nil")
	}
	return &ProfileManager{
		publisher: publisher,
		retriever: retriever,
	}
}

// PublishProfile serializes a Profile struct and publishes it to DDS.
// It returns the DDS Content ID (CID) of the published profile data.
func (pm *ProfileManager) PublishProfile(profile *Profile) (string, error) {
	if profile == nil {
		return "", fmt.Errorf("cannot publish a nil profile")
	}

	// Serialize the profile to JSON bytes
	profileData, err := profile.ToJSON()
	if err != nil {
		return "", fmt.Errorf("failed to serialize profile to JSON: %w", err)
	}

	// Publish the serialized profile data using ContentPublisher
	profileCID, err := pm.publisher.PublishContent(profileData)
	if err != nil {
		return "", fmt.Errorf("failed to publish profile data to DDS: %w", err)
	}

	fmt.Printf("ProfileManager: Profile for %s (version %d) published to DDS with CID: %s\n", profile.OwnerAddress, profile.Version, profileCID)
	return profileCID, nil
}

// RetrieveProfile fetches profile data from DDS using its CID and deserializes it.
func (pm *ProfileManager) RetrieveProfile(profileCID string) (*Profile, error) {
	if profileCID == "" {
		return nil, fmt.Errorf("profile CID cannot be empty for retrieval")
	}

	// Retrieve the serialized profile data using ContentRetriever
	profileData, err := pm.retriever.RetrieveContent(profileCID)
	if err != nil {
		return nil, fmt.Errorf("failed to retrieve profile data (CID: %s) from DDS: %w", profileCID, err)
	}

	// Deserialize the JSON data back into a Profile struct
	profile, err := FromJSON(profileData)
	if err != nil {
		return nil, fmt.Errorf("failed to deserialize profile data (CID: %s): %w", profileCID, err)
	}

	fmt.Printf("ProfileManager: Profile (CID: %s) for %s (version %d) successfully retrieved and deserialized.\n", profileCID, profile.OwnerAddress, profile.Version)
	return profile, nil
}

// UpdateAndPublishProfile first updates an existing profile struct with new data,
// then publishes the updated version to DDS.
// It returns the new CID of the updated profile.
func (pm *ProfileManager) UpdateAndPublishProfile(
	currentProfile *Profile,
	newDisplayName, newBio, newProfilePictureCID string,
) (string, *Profile, error) {
	if currentProfile == nil {
		return "", nil, fmt.Errorf("current profile cannot be nil for update")
	}

	// Keep a reference to the old version for comparison or logging if needed
	// oldVersion := currentProfile.Version

	err := currentProfile.Update(newDisplayName, newBio, newProfilePictureCID)
	if err != nil {
		return "", currentProfile, fmt.Errorf("failed to update profile struct: %w", err)
	}

	// if currentProfile.Version == oldVersion {
	// 	fmt.Printf("ProfileManager: No changes detected for profile of %s. Publishing skipped.\n", currentProfile.OwnerAddress)
	// 	// If no actual changes, we might not need to republish.
	// 	// However, the current Profile.Update increments version only if changes occurred.
	// 	// To get a new CID, it *must* be republished even if content is identical but a new version is desired.
	//  // For simplicity, we always republish if Update() didn't error.
	//  // The caller can decide if they want to republish based on whether currentProfile.Version changed.
	// }

	newCID, err := pm.PublishProfile(currentProfile)
	if err != nil {
		// Potentially revert profile struct changes if publish fails?
		// For now, the struct remains updated in memory, but publish failed.
		return "", currentProfile, fmt.Errorf("profile struct updated, but failed to publish updated profile: %w", err)
	}

	return newCID, currentProfile, nil
}
