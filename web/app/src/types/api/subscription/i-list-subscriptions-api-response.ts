import { ISubscriptionViewModel } from './i-subscription.view-model'

export interface IListSubscriptionsApiResponse {
  total: number
  results: ISubscriptionViewModel[]
}
