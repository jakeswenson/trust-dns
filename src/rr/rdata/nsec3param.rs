/*
 * Copyright (C) 2015 Benjamin Fry <benjaminfry@me.com>
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

//! parameters used for the nsec3 hash method

use ::serialize::binary::*;
use ::error::*;
use ::rr::dnssec::Nsec3HashAlgorithm;

/// [RFC 5155, NSEC3, March 2008](https://tools.ietf.org/html/rfc5155#section-4)
///
/// ```text
/// 4.  The NSEC3PARAM Resource Record
///
///    The NSEC3PARAM RR contains the NSEC3 parameters (hash algorithm,
///    flags, iterations, and salt) needed by authoritative servers to
///    calculate hashed owner names.  The presence of an NSEC3PARAM RR at a
///    zone apex indicates that the specified parameters may be used by
///    authoritative servers to choose an appropriate set of NSEC3 RRs for
///    negative responses.  The NSEC3PARAM RR is not used by validators or
///    resolvers.
///
///    If an NSEC3PARAM RR is present at the apex of a zone with a Flags
///    field value of zero, then there MUST be an NSEC3 RR using the same
///    hash algorithm, iterations, and salt parameters present at every
///    hashed owner name in the zone.  That is, the zone MUST contain a
///    complete set of NSEC3 RRs with the same hash algorithm, iterations,
///    and salt parameters.
///
///    The owner name for the NSEC3PARAM RR is the name of the zone apex.
///
///    The type value for the NSEC3PARAM RR is 51.
///
///    The NSEC3PARAM RR RDATA format is class independent and is described
///    below.
///
///    The class MUST be the same as the NSEC3 RRs to which this RR refers.
///
/// 4.2.  NSEC3PARAM RDATA Wire Format
///
///  The RDATA of the NSEC3PARAM RR is as shown below:
///
///                       1 1 1 1 1 1 1 1 1 1 2 2 2 2 2 2 2 2 2 2 3 3
///   0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
///  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
///  |   Hash Alg.   |     Flags     |          Iterations           |
///  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
///  |  Salt Length  |                     Salt                      /
///  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
///
///  Hash Algorithm is a single octet.
///
///  Flags field is a single octet.
///
///  Iterations is represented as a 16-bit unsigned integer, with the most
///  significant bit first.
///
///  Salt Length is represented as an unsigned octet.  Salt Length
///  represents the length of the following Salt field in octets.  If the
///  value is zero, the Salt field is omitted.
///
///  Salt, if present, is encoded as a sequence of binary octets.  The
///  length of this field is determined by the preceding Salt Length
///  field.
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct NSEC3PARAM{ hash_algorithm: Nsec3HashAlgorithm, opt_out: bool, iterations: u16, salt: Vec<u8> }

impl NSEC3PARAM {
  pub fn new(hash_algorithm: Nsec3HashAlgorithm, opt_out: bool, iterations: u16, salt: Vec<u8>) -> NSEC3PARAM {
    NSEC3PARAM{ hash_algorithm: hash_algorithm, opt_out: opt_out, iterations: iterations, salt: salt }
  }

  /// [RFC 5155, NSEC3, March 2008](https://tools.ietf.org/html/rfc5155#section-4.1.1)
  ///
  /// ```text
  /// 4.1.1.  Hash Algorithm
  ///
  ///    The Hash Algorithm field identifies the cryptographic hash algorithm
  ///    used to construct the hash-value.
  ///
  ///    The acceptable values are the same as the corresponding field in the
  ///    NSEC3 RR.
  /// ```
  pub fn get_hash_algorithm(&self) -> Nsec3HashAlgorithm { self.hash_algorithm }

  /// [RFC 5155, NSEC3, March 2008](https://tools.ietf.org/html/rfc5155#section-4.1.2)
  ///
  /// ```text
  /// 4.1.2.  Flag Fields
  ///
  ///    The Opt-Out flag is not used and is set to zero.
  ///
  ///    All other flags are reserved for future use, and must be zero.
  ///
  ///    NSEC3PARAM RRs with a Flags field value other than zero MUST be
  ///    ignored.
  /// ```
  pub fn is_opt_out(&self) -> bool { self.opt_out }

  /// [RFC 5155, NSEC3, March 2008](https://tools.ietf.org/html/rfc5155#section-4.1.3)
  ///
  /// ```text
  /// 4.1.3.  Iterations
  ///
  ///    The Iterations field defines the number of additional times the hash
  ///    is performed.
  ///
  ///    Its acceptable values are the same as the corresponding field in the
  ///    NSEC3 RR.
  /// ```
  pub fn get_iterations(&self) -> u16 { self.iterations }

  /// [RFC 5155, NSEC3, March 2008](https://tools.ietf.org/html/rfc5155#section-4.1.5)
  ///
  /// ```text
  /// 4.1.5.  Salt
  ///
  ///    The Salt field is appended to the original owner name before hashing.
  /// ```
  pub fn get_salt(&self) -> &[u8] { &self.salt }
}

pub fn read(decoder: &mut BinDecoder) -> DecodeResult<NSEC3PARAM> {
  let hash_algorithm = try!(Nsec3HashAlgorithm::from_u8(try!(decoder.read_u8())));
  let flags: u8 = try!(decoder.read_u8());

  if flags & 0b1111_1110 != 0 { return Err(DecodeErrorKind::UnrecognizedNsec3Flags(flags).into()) }
  let opt_out: bool = flags & 0b0000_0001 == 0b0000_0001;
  let iterations: u16 = try!(decoder.read_u16());
  let salt_len: u8 = try!(decoder.read_u8());
  let salt: Vec<u8> = try!(decoder.read_vec(salt_len as usize));

  Ok(NSEC3PARAM::new(hash_algorithm, opt_out, iterations, salt))
}

pub fn emit(encoder: &mut BinEncoder, rdata: &NSEC3PARAM) -> EncodeResult {
  try!(encoder.emit(rdata.get_hash_algorithm().into()));
  let mut flags: u8 = 0;
  if rdata.is_opt_out() { flags |= 0b0000_0001 };
  try!(encoder.emit(flags));
  try!(encoder.emit_u16(rdata.get_iterations()));
  try!(encoder.emit(rdata.get_salt().len() as u8));
  try!(encoder.emit_vec(&rdata.get_salt()));

  Ok(())
}

#[test]
pub fn test() {
  let rdata = NSEC3PARAM::new(Nsec3HashAlgorithm::SHA1, true, 2, vec![1,2,3,4,5]);

  let mut bytes = Vec::new();
  let mut encoder: BinEncoder = BinEncoder::new(&mut bytes);
  assert!(emit(&mut encoder, &rdata).is_ok());
  let bytes = encoder.as_bytes();

  println!("bytes: {:?}", bytes);

  let mut decoder: BinDecoder = BinDecoder::new(bytes);
  let read_rdata = read(&mut decoder);
  assert!(read_rdata.is_ok(), format!("error decoding: {:?}", read_rdata.unwrap_err()));
  assert_eq!(rdata, read_rdata.unwrap());
}
