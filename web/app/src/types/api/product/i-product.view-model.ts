import { BlockchainStatus } from './enum-blockchain-status'
import { PaymentCurrency } from './enum-payment-currency'
import { PriceType } from './enum-price-type'
import { ProductCategory } from './enum-product-category'
import { ProductStatus } from './enum-product-status'
import { SubscriptionInterval } from './enum-subscription-interval'

export interface IPriceViewModel {
  id: string
  active: boolean
  name: String
  price_type: PriceType
  amount: string
  base_currency: PaymentCurrency
  subscription_interval?: SubscriptionInterval
  subscription_interval_count?: number
  trial_days?: number
}

export interface IProductViewModel {
  id: string
  user_id: string
  name: string
  description: string
  blurb: string
  contract_address: string
  payout_address: string
  category: ProductCategory
  status: ProductStatus
  blockchain_status: BlockchainStatus
  transaction_hash?: string
  price?: IPriceViewModel
  created_at: Date
  updated_at: Date
}
