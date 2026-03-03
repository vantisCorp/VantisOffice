# Vantis Link User Guide

## Table of Contents
1. [Introduction](#introduction)
2. [Getting Started](#getting-started)
3. [Creating Sessions](#creating-sessions)
4. [Inviting Collaborators](#inviting-collaborators)
5. [Real-Time Collaboration](#real-time-collaboration)
6. [CRDT and Conflict Resolution](#crdt-and-conflict-resolution)
7. [Encryption and Security](#encryption-and-security)
8. [Offline Support](#offline-support)
9. [Settings and Preferences](#settings-and-preferences)
10. [Keyboard Shortcuts](#keyboard-shortcuts)
11. [Tips and Tricks](#tips-and-tricks)

## Introduction

Vantis Link is a peer-to-peer collaboration system with end-to-end encryption, using CRDT (Conflict-free Replicated Data Types) for real-time editing without central servers.

**Key Features:**
- Peer-to-peer (P2P) architecture - no central servers
- End-to-end encryption (AES-256-GCM, ChaCha20-Poly1305)
- CRDT-based conflict resolution
- Real-time collaboration
- Offline support with sync on reconnect
- Multiple transport protocols (TCP, UDP, WebRTC, QUIC)

## Getting Started

### Launching Vantis Link

```bash
cargo run --release -p vantis-link
```

### Creating Your First Session

1. Launch Vantis Link
2. Click "New Session" or press `Ctrl+N` / `Cmd+N`
3. Configure session settings
4. Share session ID with collaborators
5. Start collaborating in real-time

### Interface Overview

```
┌─────────────────────────────────────────────────────┐
│  File  Session  Collaborate  Security  Help       │
├─────────────────────────────────────────────────────┤
│  [New Session] [Join Session] [Settings]           │
├─────────────────────────────────────────────────────┤
│  Session: My Project (ID: abc123...)              │
│  Status: ● Connected | 3 collaborators            │
├─────────────────────────────────────────────────────┤
│  Participants:                                     │
│  ├─ You (Owner) ● Online                          │
│  ├─ Alice ● Online                                │
│  └─ Bob ● Online                                  │
├─────────────────────────────────────────────────────┤
│  Documents:                                        │
│  ├─ Main Document                                 │
│  ├─ Notes                                         │
│  └─ Resources                                     │
├─────────────────────────────────────────────────────┤
│  Activity:                                         │
│  ├─ Alice: Added paragraph                        │
│  ├─ Bob: Edited section 3                         │
│  └─ You: Created document                         │
└─────────────────────────────────────────────────────┘
```

## Creating Sessions

### New Session

Create a new collaboration session:

1. **Session** → **New Session** or press `Ctrl+N` / `Cmd+N`
2. Configure session settings:
   - Session name
   - Session type (public, private, password-protected)
   - Max participants
   - Encryption algorithm
3. Click **Create**

**Session Types:**
- **Public**: Anyone with session ID can join
- **Private**: Invitation only
- **Password-Protected**: Session ID + password required

### Session Settings

**General Settings:**
- Session name
- Description
- Icon/Avatar

**Security Settings:**
- Encryption algorithm:
  - AES-256-GCM (default, hardware accelerated)
  - ChaCha20-Poly1305 (software fallback)
- Key exchange: X25519
- Session timeout

**Transport Settings:**
- Protocol: TCP, UDP, WebRTC, QUIC
- Port configuration
- Proxy settings

### Joining Sessions

**Join by Session ID:**
1. **Session** → **Join Session**
2. Enter session ID
3. Enter password (if required)
4. Click **Join**

**Join by Invitation Link:**
1. Click invitation link
2. Vantis Link opens
3. Click **Join**
4. If password required, enter password

**Join by QR Code:**
1. **Session** → **Join by QR**
2. Scan QR code with camera
3. Session details populate
4. Click **Join**

## Inviting Collaborators

### Generating Invitations

**Share Session ID:**
1. Click **Share** button
2. Copy session ID
3. Send to collaborators via any channel

**Generate Invitation Link:**
1. **Session** → **Invite**
2. Click **Generate Link**
3. Copy link
4. Share via email, chat, etc.

**Create QR Code:**
1. **Session** → **Invite** → **QR Code**
2. Display QR code
3. Collaborators scan with Vantis Link
4. Link auto-opens in their app

### Managing Invitations

**View Pending Invitations:**
1. **Session** → **Invitations**
2. See all pending invitations
3. Status: Sent, Opened, Accepted, Expired

**Revoke Invitation:**
1. Select invitation
2. Click **Revoke**
3. Invitation becomes invalid

**Set Expiration:**
1. Create invitation
2. Set expiration time
3. Options: 1 hour, 24 hours, 7 days, never

## Real-Time Collaboration

### Document Collaboration

**Creating Documents:**
1. **Collaborate** → **New Document**
2. Choose document type
3. Document syncs automatically to all participants

**Editing Together:**
- See collaborators' cursors in real-time
- View typing as it happens
- Changes sync instantly
- No "saving" required - auto-sync

**Cursors and Presence:**
- Each collaborator has colored cursor
- Names appear above cursors
- Selection highlights show what others are selecting
- Activity indicators show who is active

### Communication

**Text Chat:**
1. **Collaborate** → **Chat** or press `Ctrl+Shift+C` / `Cmd+Shift+C`
2. Type message
3. Press Enter to send
4. Chat history preserved for session

**Voice Chat:**
1. **Collaborate** → **Voice Chat**
2. Click microphone to mute/unmute
3. Click headphones to mute audio
4. See who is speaking

**Video Chat:**
1. **Collaborate** → **Video Chat**
2. Enable camera
3. See video thumbnails
4. Click to expand video

### Sharing Files

**Send File:**
1. **Collaborate** → **Share File**
2. Select file
3. Choose recipients (all or selected)
4. File encrypted and sent P2P

**Receive File:**
1. Notification appears
2. Click to accept or reject
3. File downloads to designated folder
4. Open or save to different location

## CRDT and Conflict Resolution

### What is CRDT?

CRDT (Conflict-free Replicated Data Types) enables collaboration without central servers:

**Benefits:**
- No merge conflicts
- Works offline
- Eventually consistent
- No need for locking

**How it Works:**
1. Each edit gets a unique timestamp and vector clock
2. Edits propagate to all peers
3. Concurrent edits merge automatically
4. Resolution rules ensure consistency

### Conflict Resolution

**Automatic Resolution:**
- Last-Writer-Wins (LWW) for simple values
- Multi-Value Register for concurrent updates
- Conflict-free merge for text (RGA algorithm)

**Viewing Conflicts:**
1. **Collaborate** → **Conflict History**
2. See all concurrent edits
3. View how they were resolved
4. No manual intervention needed

**Forcing Resolution (Advanced):**
1. **Collaborate** → **Advanced** → **Resolve Conflicts**
2. View pending conflicts
3. Choose resolution strategy
4. Apply manually if needed

### Sync Status

**Check Sync Status:**
1. View status bar
2. Shows sync status:
   - ● Green: Synced
   - ● Yellow: Syncing
   - ● Red: Connection issues

**Sync History:**
1. **Collaborate** → **Sync History**
2. See all sync events
3. View timing and peer information
4. Debug sync issues

## Encryption and Security

### End-to-End Encryption

All communication is end-to-end encrypted:

**Encryption Flow:**
1. Each peer generates key pair (X25519)
2. Public keys exchanged
3. Shared secret derived
4. All messages encrypted with shared secret

**Supported Algorithms:**
- **AES-256-GCM**: Hardware accelerated, fast
- **ChaCha20-Poly1305**: Software implementation, compatible

### Key Management

**View Your Keys:**
1. **Security** → **My Keys**
2. View:
   - Public key (shareable)
   - Key fingerprint
   - Key creation date
   - Associated identity

**Export Public Key:**
1. **Security** → **My Keys** → **Export**
2. Choose format
3. Share public key with collaborators

**Trust Keys:**
1. When joining session, verify peer keys
2. Compare fingerprints out-of-band
3. Mark keys as trusted
4. Warn if key changes

### Security Settings

**Encryption Level:**
1. **Security** → **Settings**
2. Choose algorithm
3. Set key rotation interval

**Authentication:**
1. **Security** → **Authentication**
2. Configure:
   - Require key verification
   - Auto-trust known peers
   - Warn on new keys

**Audit Log:**
1. **Security** → **Audit Log**
2. View all security events:
   - Peers joined/left
   - Key exchanges
   - Messages sent/received
   - Errors and warnings

## Offline Support

### Working Offline

Vantis Link works without internet:

**Offline Mode:**
1. Continue editing documents
2. Changes stored locally
3. Timestamped for later sync
4. No data loss

**Offline Indicator:**
- Status bar shows offline status
- Yellow indicator: Offline
- Red indicator: Connection lost

### Sync on Reconnect

**Automatic Sync:**
1. When connection restored
2. Local changes uploaded
3. Remote changes downloaded
4. CRDT ensures no conflicts

**Sync Progress:**
- Progress bar appears
- Shows upload/download status
- Completes automatically

**Resolve Offline Conflicts:**
1. If major divergence
2. Manual merge may be needed
3. Side-by-side comparison
4. Choose which changes to keep

## Settings and Preferences

### General Settings

**Identity:**
- Display name
- Avatar
- Email (optional)

**Network:**
- Preferred protocol (TCP, UDP, WebRTC, QUIC)
- Port range
- STUN/TURN servers

**Notifications:**
- New message notifications
- Peer join/leave
- File received
- Mention notifications

### Appearance

**Theme:**
- Light
- Dark
- System default

**Font:**
- Font family
- Font size
- Line height

**Layout:**
- Compact mode
- Sidebar position
- Panel visibility

### Privacy

**Data Collection:**
- Analytics (on/off)
- Error reporting (on/off)

**Peer Discovery:**
- Allow discovery by email
- Allow discovery by phone
- Show online status

## Keyboard Shortcuts

### Session Management

| Action | Windows/Linux | macOS |
|--------|---------------|-------|
| New Session | `Ctrl+N` | `Cmd+N` |
| Join Session | `Ctrl+J` | `Cmd+J` |
| Leave Session | `Ctrl+Shift+J` | `Cmd+Shift+J` |
| Invite | `Ctrl+I` | `Cmd+I` |
| Share | `Ctrl+Shift+S` | `Cmd+Shift+S` |

### Navigation

| Action | Windows/Linux | macOS |
|--------|---------------|-------|
| Toggle Chat | `Ctrl+Shift+C` | `Cmd+Shift+C` |
| Toggle Participants | `Ctrl+Shift+P` | `Cmd+Shift+P` |
| Toggle Activity | `Ctrl+Shift+A` | `Cmd+Shift+A` |
| Settings | `Ctrl+,` | `Cmd+,` |

### Communication

| Action | Windows/Linux | macOS |
|--------|---------------|-------|
| Send Message | `Enter` | `Enter` |
| New Line | `Shift+Enter` | `Shift+Enter` |
| Toggle Mute | `Ctrl+M` | `Cmd+M` |
| Toggle Deafen | `Ctrl+D` | `Cmd+D` |

## Tips and Tricks

### Collaboration Tips

1. **Use Consistent Naming**: Help collaborators identify you
2. **Share Session Links**: Easier than sharing session IDs
3. **Set Ground Rules**: Establish editing conventions
4. **Use Comments**: Add comments for discussion
5. **Review History**: Check edit history for context

### Performance Tips

1. **Reduce File Sizes**: Compress files before sharing
2. **Limit Participants**: Fewer peers = faster sync
3. **Use Ethernet**: More stable than Wi-Fi
4. **Close Unused Sessions**: Free up resources
5. **Check Network**: Ensure stable connection

### Security Tips

1. **Verify Keys**: Always verify peer keys
2. **Use Strong Passwords**: For password-protected sessions
3. **Review Access**: Check who has access
4. **Revoke Old Invites**: Clean up expired invitations
5. **Audit Regularly**: Review audit logs

### Troubleshooting Tips

1. **Check Connection**: Verify internet connection
2. **Restart Session**: Leave and rejoin
3. **Check Firewall**: Ensure ports are open
4. **Update App**: Use latest version
5. **Check Logs**: Review error logs

## Troubleshooting

### Common Issues

**Cannot Connect:**
- Check internet connection
- Verify session ID
- Check firewall settings
- Try different protocol

**Sync Issues:**
- Check connection status
- Verify peer is online
- Check for large pending changes
- Restart session

**Encryption Errors:**
- Verify peer keys
- Check key trust status
- Reset keys if needed
- Ensure same encryption algorithm

**Performance Issues:**
- Reduce number of participants
- Limit shared files
- Check network speed
- Close other applications

### Getting Help

- Check documentation for specific features
- Review security settings
- Report issues at: https://github.com/vantisCorp/VantisOffice/issues

---

**Last Updated**: 2024-03-03  
**VantisLink Version**: 0.2.0