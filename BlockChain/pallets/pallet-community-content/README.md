# Community Content Pallet

A Substrate pallet for managing community-generated content in the CritterCraft Universe.

## Overview

The Community Content pallet enables users to create, share, and monetize their own content within the CritterCraft ecosystem. It provides a complete system for content submission, moderation, and royalty management.

## Features

- **Content Creation**: Users can submit various types of content including critter skins, accessories, item designs, environment themes, and quest templates
- **Content Moderation**: A moderation system ensures quality and appropriate content
- **Royalty Management**: Creators earn royalties when their content is used or purchased
- **Content Updates**: Approved content can be updated by its creator
- **Usage Tracking**: Track purchases and usage of content

## Extrinsics

### For Content Creators

- `submit_content`: Submit new content for moderation
- `update_content`: Update existing approved content

### For Moderators

- `moderate_content`: Approve, reject, or flag content

### For System Integration

- `record_purchase`: Record a content purchase (called by marketplace)
- `record_usage`: Record content usage (called by game logic)

### For Governance

- `add_moderator`: Add a new moderator (root only)
- `remove_moderator`: Remove a moderator (root only)

## Storage

- `Content`: Maps ContentId to ContentDetails
- `ContentDescriptions`: Stores content descriptions
- `ContentUris`: Stores content URIs
- `ContentModeration`: Stores moderation details
- `CreatorContent`: Maps creators to their content
- `PendingContent`: List of content pending moderation
- `ApprovedContent`: List of approved content
- `FlaggedContent`: List of flagged content
- `ContentByType`: Maps content types to content IDs
- `Moderators`: Set of accounts with moderation privileges

## Integration with Marketplace

The Community Content pallet implements the `ContentRoyaltyManager` trait, which allows the Marketplace pallet to:

1. Verify content ownership
2. Determine royalty percentages
3. Pay royalties to content creators

## Configuration

The pallet is configurable through its `Config` trait:

- `MaxNameLength`: Maximum length for content names
- `MaxDescriptionLength`: Maximum length for content descriptions
- `MaxUriLength`: Maximum length for content URIs
- `MaxReasonLength`: Maximum length for moderation reasons
- `ContentSubmissionDeposit`: Required deposit for content submission
- `MaxRoyaltyPercentage`: Maximum allowed royalty percentage (0-100)
- `CommunityTreasuryAccountId`: Account for the community treasury

## Content Lifecycle

1. **Submission**: Creator submits content and pays a deposit
2. **Moderation**: Content is reviewed by moderators
3. **Approval/Rejection**: Content is either approved (deposit returned) or rejected (deposit slashed)
4. **Listing**: Approved content can be listed in the marketplace
5. **Purchase/Usage**: When content is purchased or used, royalties are paid to the creator
6. **Updates**: Creator can update approved content

## Security

- Deposits prevent spam submissions
- Moderation ensures quality and appropriate content
- Only the content creator can update their content
- Only designated moderators can approve/reject content
- Root account controls moderator designation