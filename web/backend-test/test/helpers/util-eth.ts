import { Wallet } from 'ethers'

export const USER1_ADDRESS = '0x871dF04f23EA830C3b65e16D76433004EbeDd001'
export const USER1_PRIVATE_KEY =
  'e447796ef23dbf00aac8f6ee6552c3d24c52d45a09e58102151bbcfcda862b0a'

// A real address/private key for testing
export const TEST_ADDRESS1 = '0x525A4073E13c13B79C6B29de8a7B5639458a8002'
export const TEST_PRIVATE_KEY1 =
  '56d1c79ca3ba77b3f2cc5144c2e8788892af85c4f37064473ab45bd3378e553d'

export const TEST_ADDRESS2 = '0x6d0672462E83bA94A1eb9EDb3a72f860F0Bef003'
export const TEST_PRIVATE_KEY2 =
  'f180107ff4ab6d2f6421a077ed627a76e3f6372aacf8d71c0ae59a3ca118e6c6'

export const signMessage = (key: string, message: string): Promise<string> => {
  const signer = new Wallet(key)
  return signer.signMessage(message)
}

export const registerSignature = (key: string, address: string) => {
  const message = `Register a Paynetic account with ${address}`
  return signMessage(key, message)
}
