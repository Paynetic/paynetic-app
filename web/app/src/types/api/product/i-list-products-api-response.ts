import { IProductViewModel } from './i-product.view-model'

export interface IListProductsApiResponse {
  total: number
  results: IProductViewModel[]
}
