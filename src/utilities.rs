/*Author: Lerato Mokoena
  Company: Arithoptix Pty Ltd (Mural Code).
  Date: June 23, 2024
  Description: Utility function
 */
use hex;

pub fn encode_to_hex(data: &[u8]) -> String {
    hex::encode(data)
}
