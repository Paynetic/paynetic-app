import { PriceType } from './enum-price-type'
import { ProductCategory } from './enum-product-category'
import { SubscriptionInterval } from './enum-subscription-interval'

export interface ICreatePriceDto {
  name: string
  price_type: PriceType
  amount: string
  subscription_interval?: SubscriptionInterval
  subscription_interval_count?: number
  trial_days?: number
}

export interface ICreateProductApiRequest {
  name: string
  description: string
  blurb: string
  category: ProductCategory
  price?: ICreatePriceDto
}
