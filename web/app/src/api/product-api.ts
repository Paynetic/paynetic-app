import {
  IGetProductApiResponse,
  IListProductsApiRequest,
  IListProductsApiResponse,
} from '@app/types'
import { rootApi } from './root-api'
import { RequestParams } from '@samatech/fetch-api'

export const apiListProducts = async (
  query: IListProductsApiRequest,
): Promise<IListProductsApiResponse> => {
  const { data } = await rootApi.authRequest<IListProductsApiResponse>({
    url: 'products',
    method: 'GET',
    params: query as unknown as RequestParams,
  })
  return data
}

export const apiGetProduct = async (id: string): Promise<IGetProductApiResponse> => {
  const { data } = await rootApi.authRequest<IGetProductApiResponse>({
    url: `products/${id}`,
    method: 'GET',
  })
  return data
}
