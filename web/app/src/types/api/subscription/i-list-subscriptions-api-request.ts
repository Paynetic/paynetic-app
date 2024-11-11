import { SortDirection } from '../shared'
import { SubscriptionStatus } from './enum-subscription-status'

export interface IListSubscriptionsApiRequest {
  readonly from?: number
  readonly to?: number
  statuses?: SubscriptionStatus[]
  user_id?: string
  product_id?: string
  column?: 'amount' | 'created_at' | 'updated_at'
  direction?: SortDirection
}
