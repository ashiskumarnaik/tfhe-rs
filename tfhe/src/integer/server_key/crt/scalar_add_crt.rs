use crate::integer::server_key::CheckError;
use crate::integer::{CrtCiphertext, ServerKey};

impl ServerKey {
    /// Computes homomorphically an addition between a scalar and a ciphertext.
    ///
    /// This function computes the operation without checking if it exceeds the capacity of the
    /// ciphertext.
    ///
    /// The result is returned as a new ciphertext.
    ///
    /// # Example
    ///
    ///```rust
    /// use tfhe::integer::gen_keys_crt;
    /// use tfhe::shortint::parameters::PARAM_MESSAGE_3_CARRY_3_KS_PBS_GAUSSIAN_2M128;
    ///
    /// // Generate the client key and the server key:
    /// let basis = vec![2, 3, 5];
    /// let modulus: u64 = basis.iter().product();
    /// let (cks, sks) = gen_keys_crt(PARAM_MESSAGE_3_CARRY_3_KS_PBS_GAUSSIAN_2M128, basis);
    ///
    /// let clear_1 = 14;
    /// let clear_2 = 14;
    /// // Encrypt two messages
    /// let mut ctxt_1 = cks.encrypt(clear_1);
    ///
    /// sks.unchecked_crt_scalar_add_assign(&mut ctxt_1, clear_2);
    ///
    /// // Decrypt
    /// let res = cks.decrypt(&ctxt_1);
    /// assert_eq!((clear_1 + clear_2) % modulus, res);
    /// ```
    pub fn unchecked_crt_scalar_add(&self, ct: &CrtCiphertext, scalar: u64) -> CrtCiphertext {
        let mut result = ct.clone();
        self.unchecked_crt_scalar_add_assign(&mut result, scalar);
        result
    }

    /// Computes homomorphically an addition between a scalar and a ciphertext.
    ///
    /// This function computes the operation without checking if it exceeds the capacity of the
    /// ciphertext.
    ///
    /// The result is assigned to the `ct_left` ciphertext.
    pub fn unchecked_crt_scalar_add_assign(&self, ct: &mut CrtCiphertext, scalar: u64) {
        //Add the crt representation of the scalar to the ciphertext
        for (ct_i, mod_i) in ct.blocks.iter_mut().zip(ct.moduli.iter()) {
            let scalar_i = scalar % mod_i;

            self.key.unchecked_scalar_add_assign(ct_i, scalar_i as u8);
        }
    }

    /// Verifies if a scalar can be added to a ciphertext.
    ///
    /// # Example
    ///
    ///```rust
    /// use tfhe::integer::gen_keys_crt;
    /// use tfhe::shortint::parameters::PARAM_MESSAGE_3_CARRY_3_KS_PBS_GAUSSIAN_2M128;
    ///
    /// // Generate the client key and the server key:
    /// let basis = vec![2, 3, 5];
    /// let (cks, sks) = gen_keys_crt(PARAM_MESSAGE_3_CARRY_3_KS_PBS_GAUSSIAN_2M128, basis);
    ///
    /// let clear_1 = 14;
    /// let clear_2 = 14;
    /// // Encrypt two messages
    /// let ctxt_1 = cks.encrypt(clear_1);
    ///
    /// sks.is_crt_scalar_add_possible(&ctxt_1, clear_2).unwrap();
    /// ```
    pub fn is_crt_scalar_add_possible(
        &self,
        ct: &CrtCiphertext,
        scalar: u64,
    ) -> Result<(), CheckError> {
        for (ct_i, mod_i) in ct.blocks.iter().zip(ct.moduli.iter()) {
            let scalar_i = scalar % mod_i;

            self.key
                .is_scalar_add_possible(ct_i.noise_degree(), scalar_i as u8)?;
        }

        Ok(())
    }

    /// Computes homomorphically an addition between a scalar and a ciphertext.
    ///
    /// If the operation can be performed, the result is returned in a new ciphertext.
    /// Otherwise a [CheckError] is returned.
    ///
    /// # Example
    ///
    /// ```rust
    /// use tfhe::integer::gen_keys_crt;
    /// use tfhe::shortint::parameters::PARAM_MESSAGE_3_CARRY_3_KS_PBS_GAUSSIAN_2M128;
    ///
    /// // Generate the client key and the server key:
    /// let basis = vec![2, 3, 5];
    /// let modulus: u64 = basis.iter().product();
    /// let (cks, sks) = gen_keys_crt(PARAM_MESSAGE_3_CARRY_3_KS_PBS_GAUSSIAN_2M128, basis);
    ///
    /// let clear_1 = 14;
    /// let clear_2 = 14;
    /// // Encrypt two messages
    /// let mut ctxt_1 = cks.encrypt(clear_1);
    ///
    /// sks.checked_crt_scalar_add_assign(&mut ctxt_1, clear_2)
    ///     .unwrap();
    ///
    /// // Decrypt
    /// let res = cks.decrypt(&ctxt_1);
    /// assert_eq!((clear_1 + clear_2) % modulus, res);
    /// ```
    pub fn checked_crt_scalar_add(
        &self,
        ct: &CrtCiphertext,
        scalar: u64,
    ) -> Result<CrtCiphertext, CheckError> {
        self.is_crt_scalar_add_possible(ct, scalar)?;
        Ok(self.unchecked_crt_scalar_add(ct, scalar))
    }

    /// Computes homomorphically an addition between a scalar and a ciphertext.
    ///
    /// If the operation can be performed, the result is stored in the `ct_left` ciphertext.
    /// Otherwise a [CheckError] is returned, and `ct_left` is not modified.
    pub fn checked_crt_scalar_add_assign(
        &self,
        ct: &mut CrtCiphertext,
        scalar: u64,
    ) -> Result<(), CheckError> {
        self.is_crt_scalar_add_possible(ct, scalar)?;
        self.unchecked_crt_scalar_add_assign(ct, scalar);
        Ok(())
    }

    /// Computes homomorphically the addition of ciphertext with a scalar.
    ///
    /// The result is returned in a new ciphertext.
    ///
    /// # Example
    ///
    ///```rust
    /// use tfhe::integer::gen_keys_crt;
    /// use tfhe::shortint::parameters::PARAM_MESSAGE_3_CARRY_3_KS_PBS_GAUSSIAN_2M128;
    ///
    /// // Generate the client key and the server key:
    /// let basis = vec![2, 3, 5];
    /// let modulus: u64 = basis.iter().product();
    /// let (cks, sks) = gen_keys_crt(PARAM_MESSAGE_3_CARRY_3_KS_PBS_GAUSSIAN_2M128, basis);
    ///
    /// let clear_1 = 14;
    /// let clear_2 = 14;
    /// // Encrypt two messages
    /// let mut ctxt_1 = cks.encrypt(clear_1);
    ///
    /// let ctxt = sks.smart_crt_scalar_add(&mut ctxt_1, clear_2);
    ///
    /// // Decrypt
    /// let res = cks.decrypt(&ctxt);
    /// assert_eq!((clear_1 + clear_2) % modulus, res);
    /// ```
    pub fn smart_crt_scalar_add(&self, ct: &mut CrtCiphertext, scalar: u64) -> CrtCiphertext {
        if self.is_crt_scalar_add_possible(ct, scalar).is_err() {
            self.full_extract_message_assign(ct);
        }

        self.is_crt_scalar_add_possible(ct, scalar).unwrap();

        let mut ct = ct.clone();
        self.unchecked_crt_scalar_add_assign(&mut ct, scalar);
        ct
    }

    /// Computes homomorphically the addition of ciphertext with a scalar.
    ///
    /// The result is assigned to the `ct_left` ciphertext.
    ///
    /// # Example
    ///
    ///```rust
    /// use tfhe::integer::gen_keys_crt;
    /// use tfhe::shortint::parameters::PARAM_MESSAGE_3_CARRY_3_KS_PBS_GAUSSIAN_2M128;
    ///
    /// // Generate the client key and the server key:
    /// let basis = vec![2, 3, 5];
    /// let modulus: u64 = basis.iter().product();
    /// let (cks, sks) = gen_keys_crt(PARAM_MESSAGE_3_CARRY_3_KS_PBS_GAUSSIAN_2M128, basis);
    ///
    /// let clear_1 = 14;
    /// let clear_2 = 14;
    /// // Encrypt two messages
    /// let mut ctxt_1 = cks.encrypt(clear_1);
    ///
    /// sks.smart_crt_scalar_add_assign(&mut ctxt_1, clear_2);
    ///
    /// // Decrypt
    /// let res = cks.decrypt(&ctxt_1);
    /// assert_eq!((clear_1 + clear_2) % modulus, res);
    /// ```
    pub fn smart_crt_scalar_add_assign(&self, ct: &mut CrtCiphertext, scalar: u64) {
        if self.is_crt_scalar_add_possible(ct, scalar).is_err() {
            self.full_extract_message_assign(ct);
        }

        self.is_crt_scalar_add_possible(ct, scalar).unwrap();

        self.unchecked_crt_scalar_add_assign(ct, scalar);
    }
}
