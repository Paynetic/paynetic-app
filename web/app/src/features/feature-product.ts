import { apiGetProduct, apiListProducts } from '@app/api'
import { IListProductsApiRequest, IProductViewModel } from '@app/types'
import { errorToKey } from '@app/util'
import { IFeatureParams } from './i-feature-params'

export interface IListProductParams extends IFeatureParams {
  products: IProductViewModel[]
}

export interface IGetProductParams extends IFeatureParams {
  product: IProductViewModel | undefined
}

export const listProducts = async (
  payload: IListProductsApiRequest,
  params: IListProductParams,
) => {
  params.error = undefined
  params.loading = true
  try {
    const res = await apiListProducts(payload)
    params.products = res.results
  } catch (e) {
    params.error = errorToKey(e)
  }
  params.loading = false
}

export const getProduct = async (id: string, params: IGetProductParams) => {
  params.error = undefined
  params.loading = true
  try {
    const res = await apiGetProduct(id)
    params.product = res
  } catch (e) {
    params.error = errorToKey(e)
  }
  params.loading = false
}
