import {
  BlockchainStatus,
  IGetSubscriptionApiResponse,
  PaymentCurrency,
  PriceType,
  SubscriptionInterval,
  SubscriptionStatus,
} from '@app/types'
import { testagent, TestAgent, AppDbResetService } from '../helpers'
import { describe, expect, test, beforeAll, beforeEach } from 'vitest'
import { adminAuthHeader, userAuthHeader } from '../helpers'
import { testConfig } from '../test.config'
import { commonRegex } from '@app/util'

describe('Get User', () => {
  const testEndpoint = '/api/subscriptions'
  let api: TestAgent
  let testHelperApiUrl: string
  let dbResetService: AppDbResetService
  let adminAuth: string
  let subscriptionId: string

  beforeAll(() => {
    api = testagent(testConfig.get('apiUrl'))
    testHelperApiUrl = testConfig.get('apiTestHelperUrl')
    dbResetService = new AppDbResetService(testHelperApiUrl)
    adminAuth = adminAuthHeader()
  })

  beforeEach(async () => {
    await dbResetService.resetDb()
    subscriptionId = 'daf969f2-740a-4a4e-ab41-b9ba31e32bbb'
  })

  describe('when requester is Admin', () => {
    test('returns 200 and subscription', async () => {
      const response = await api
        .get(`${testEndpoint}/${subscriptionId}`)
        .set('Authorization', adminAuth)
        .expect(200)

      const body: IGetSubscriptionApiResponse = response.body
      expect(body.id).toEqual(subscriptionId)
      expect(body.user_id).toEqual('90679368-ba27-4d7f-be85-849b4328d93a')
      expect(body.product_id).toEqual('673fad5b-626a-4a74-8cbd-afd45c4cf7d3')
      expect(body.contract_address).toEqual('0x0000000000000000000000000000000000000000')
      expect(body.status).toEqual(SubscriptionStatus.Trial)
      expect(body.fee_percent).toEqual(100)
      expect(body.start_time).toBeDefined()
      expect(body.current_start).toBeDefined()
      expect(body.current_end).toBeDefined()
      expect(body.grace).toEqual(5 * 24 * 60 * 60)
      expect(body.blockchain_status).toEqual(BlockchainStatus.None)
      expect(body.transaction_hash).toBeNull()
      expect(body.prices).toEqual([
        {
          id: '7f30f816-3990-4152-8693-9137e96331ec',
          active: true,
          amount: '7000000000000000',
          base_currency: PaymentCurrency.Ethereum,
          // Name excluded from subscription price
          name: '',
          price_type: PriceType.Subscription,
          subscription_interval: SubscriptionInterval.Month,
          subscription_interval_count: 1,
          trial_days: 15,
        },
      ])
      expect(body.created_at).toMatch(new RegExp(commonRegex.date))
      expect(body.updated_at).toMatch(new RegExp(commonRegex.date))
    })
  })

  describe('when requester is User', () => {
    let userAuth: string

    beforeEach(() => {
      userAuth = userAuthHeader('90679368-ba27-4d7f-be85-849b4328d93a')
      subscriptionId = 'daf969f2-740a-4a4e-ab41-b9ba31e32bbb'
    })

    test('returns 200 and subscription when getting own subscription', async () => {
      const response = await api
        .get(`${testEndpoint}/${subscriptionId}`)
        .set('Authorization', userAuth)
        .expect(200)
      const body: IGetSubscriptionApiResponse = response.body

      expect(body.id).toEqual(subscriptionId)
      expect(body.user_id).toEqual('90679368-ba27-4d7f-be85-849b4328d93a')
      expect(body.created_at).toMatch(new RegExp(commonRegex.date))
      expect(body.updated_at).toMatch(new RegExp(commonRegex.date))
    })

    test('returns 400 and subscription when getting other active subscription', async () => {
      const otherSubscriptionId = '4aff4df5-e17b-4a23-8d58-bc2d376ff022'
      await api
        .get(`${testEndpoint}/${otherSubscriptionId}`)
        .set('Authorization', userAuth)
        .expect(403, {
          code: 'None',
          message: 'Forbidden',
          status: 403,
        })
    })
  })

  test('returns 403 error when getting a subscription', async () => {
    const nonActiveId = 'c55864f1-8a47-4de2-9dd7-80d06479e70f'
    await api.get(`${testEndpoint}/${nonActiveId}`).expect(401, {
      code: 'Unauthorized',
      message: 'Unauthorized',
      status: 401,
    })
  })

  test('returns 404 when subscription does not exist', async () => {
    const noneExistId = 'caf44431-1f78-45c8-ad5f-c04ff3875092'
    await api
      .get(`${testEndpoint}/${noneExistId}`)
      .set('Authorization', adminAuth)
      .expect(404, {
        code: 'None',
        message: 'Not found',
        status: 404,
      })
  })
})
