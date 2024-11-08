import { SortDirection } from '../shared'
import { ProductCategory } from './enum-product-category'
import { ProductStatus } from './enum-product-status'

export interface IListProductsApiRequest {
  readonly from?: number
  readonly to?: number
  categories?: ProductCategory[]
  statuses?: ProductStatus[]
  user_id?: string
  column?: 'amount' | 'created_at' | 'updated_at'
  direction?: SortDirection
}
