import { PriceType } from './enum-price-type'
import { ProductCategory } from './enum-product-category'
import { ProductStatus } from './enum-product-status'
import { SubscriptionInterval } from './enum-subscription-interval'

export interface UpdatePriceDto {
  id: string
  active?: boolean
  name?: string
  price_type?: PriceType
  amount?: string
  subscription_interval?: SubscriptionInterval
  subscription_interval_count?: number
  trial_days?: number
}

export interface IUpdateProductApiRequest {
  name?: string
  description?: string
  blurb?: string
  payout_address?: string
  category?: ProductCategory
  status?: ProductStatus
  price?: UpdatePriceDto
}
