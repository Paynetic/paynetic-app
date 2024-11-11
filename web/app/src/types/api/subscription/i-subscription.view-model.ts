import {
  BlockchainStatus,
  PaymentCurrency,
  PriceType,
  SubscriptionInterval,
} from '../product'
import { SubscriptionStatus } from './enum-subscription-status'

export interface IPriceRelationViewModel {
  id: string
  active: boolean
  price_type: PriceType
  amount: string
  base_currency: PaymentCurrency
  subscription_interval?: SubscriptionInterval
  subscription_interval_count?: number
  trial_days?: number
}

export interface ISubscriptionViewModel {
  id: string
  user_id: string
  product_id: string
  contract_address: string
  status: SubscriptionStatus
  prices: IPriceRelationViewModel[]
  fee_percent: number
  start_time: number
  current_start: number
  current_end: number
  grace: number
  blockchain_status: BlockchainStatus
  transaction_hash?: string
  created_at: Date
  updated_at: Date
}
