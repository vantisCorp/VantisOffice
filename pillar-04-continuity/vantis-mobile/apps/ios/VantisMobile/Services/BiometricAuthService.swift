//
//  BiometricAuthService.swift
//  VantisMobile
//
//  Biometric authentication service
//

import Foundation
import LocalAuthentication

/// Biometric authentication service
class BiometricAuthService: ObservableObject {
    
    // MARK: - Published Properties
    
    @Published private(set) var isAvailable: Bool = false
    @Published private(set) var biometricType: LABiometricType = .none
    
    // MARK: - Initialization
    
    init() {
        checkAvailability()
    }
    
    // MARK: - Public Methods
    
    /// Check if biometric authentication is available
    func checkAvailability() {
        let context = LAContext()
        var error: NSError?
        
        isAvailable = context.canEvaluatePolicy(.deviceOwnerAuthenticationWithBiometrics, error: &error)
        biometricType = context.biometryType
    }
    
    /// Authenticate user with biometrics
    func authenticate(reason: String, completion: @escaping (Result<Void, Error>) -> Void) {
        let context = LAContext()
        context.localizedCancelTitle = "Cancel"
        
        context.evaluatePolicy(.deviceOwnerAuthenticationWithBiometrics, localizedReason: reason) { success, error in
            DispatchQueue.main.async {
                if success {
                    completion(.success(()))
                } else if let error = error {
                    completion(.failure(error))
                }
            }
        }
    }
    
    /// Get biometric type description
    var biometricTypeName: String {
        switch biometricType {
        case .none:
            return "None"
        case .touchID:
            return "Touch ID"
        case .faceID:
            return "Face ID"
        @unknown default:
            return "Biometric"
        }
    }
    
    /// Check if Face ID is available
    var hasFaceID: Bool {
        return biometricType == .faceID
    }
    
    /// Check if Touch ID is available
    var hasTouchID: Bool {
        return biometricType == .touchID
    }
    
    /// Get localized reason for authentication
    func authenticationReason(for action: String) -> String {
        if hasFaceID {
            return "Use Face ID to \(action)"
        } else if hasTouchID {
            return "Use Touch ID to \(action)"
        } else {
            return "Authenticate to \(action)"
        }
    }
}