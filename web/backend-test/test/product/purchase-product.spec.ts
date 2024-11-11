import {
  ICreateProductApiRequest,
  IPurchaseProductApiResponse,
  IGetSubscriptionApiResponse,
  PriceType,
  ProductCategory,
  SubscriptionInterval,
  SubscriptionStatus,
  BlockchainStatus,
} from '@app/types'
import { commonRegex } from '@app/util'
import {
  adminAuthHeader,
  AppDbResetService,
  testagent,
  TestAgent,
  userAuthHeader,
} from '../helpers'
import { testConfig } from '../test.config'
import { describe, expect, test, beforeAll, beforeEach } from 'vitest'

describe('Purchase Product', () => {
  const testEndpoint = (id: string) => `/api/products/${id}/purchase`
  let api: TestAgent
  let testHelperApiUrl: string
  let dbResetService: AppDbResetService
  let productId: string
  let adminAuth: string
  let userId: string
  let userAuth: string

  beforeAll(() => {
    api = testagent(testConfig.get('apiUrl'))
    testHelperApiUrl = testConfig.get('apiTestHelperUrl')
    dbResetService = new AppDbResetService(testHelperApiUrl)
    adminAuth = adminAuthHeader()
  })

  beforeEach(async () => {
    await dbResetService.resetDb()
    userId = '2199c39f-4fb3-4f72-b685-fe62b90fcef0'
    userAuth = userAuthHeader(userId)
    productId = '673fad5b-626a-4a74-8cbd-afd45c4cf7d3'
  })

  const verifySubscription = async (
    id: string,
    auth: string,
  ): Promise<IGetSubscriptionApiResponse> => {
    const response = await api
      .get(`/api/subscriptions/${id}`)
      .set('Authorization', auth)
      .expect(200)

    const body: IGetSubscriptionApiResponse = response.body
    expect(body.id).toEqual(id)
    expect(body.product_id).toEqual(productId)
    expect(body.contract_address).toBeDefined()
    expect(body.status).toEqual(SubscriptionStatus.Initial)
    expect(body.start_time).toBeDefined()
    expect(body.current_start).toBeDefined()
    expect(body.current_end).toBeDefined()
    expect(body.grace).toBeDefined()
    expect(body.blockchain_status).toEqual(BlockchainStatus.None)
    expect(body.transaction_hash).toBeNull()
    expect(body.prices).toHaveLength(1)
    expect(body.created_at).toBeDefined()
    expect(body.updated_at).toBeDefined()

    return body
  }

  test('admin purchases product', async () => {
    const response = await api
      .post(testEndpoint(productId))
      .set('Authorization', adminAuth)
      .expect(201)
    const body: IPurchaseProductApiResponse = response.body

    expect(body.id).toMatch(new RegExp(commonRegex.uuid))
    await verifySubscription(body.id, adminAuth)
  })

  test('user purchases product', async () => {
    const response = await api
      .post(testEndpoint(productId))
      .set('Authorization', userAuth)
      .expect(201)
    const body: IPurchaseProductApiResponse = response.body

    expect(body.id).toMatch(new RegExp(commonRegex.uuid))
    await verifySubscription(body.id, userAuth)
  })

  describe('when request is not valid', () => {
    test('when product state is not active', async () => {
      // Unavailable
      productId = 'd105b284-e516-42c1-8690-6612e7cd7bc7'
      await api.post(testEndpoint(productId)).set('Authorization', userAuth).expect(400, {
        code: 'ProductInactive',
        message: 'Product cannot be purchased',
        status: 400,
      })

      // Approved
      productId = 'a8708f5f-8b91-4cd2-a0e4-bbf1859a52db'
      await api.post(testEndpoint(productId)).set('Authorization', userAuth).expect(400, {
        code: 'ProductInactive',
        message: 'Product cannot be purchased',
        status: 400,
      })

      // Denied
      productId = '49d5b949-e4a1-4c21-a502-8b1d310698dc'
      await api.post(testEndpoint(productId)).set('Authorization', userAuth).expect(400, {
        code: 'ProductInactive',
        message: 'Product cannot be purchased',
        status: 400,
      })

      // No price/initial
      productId = '2f46f1b9-0a24-4a68-bbc7-a26895a54e4d'
      await api.post(testEndpoint(productId)).set('Authorization', userAuth).expect(400, {
        code: 'ProductNoPrice',
        message: 'Product cannot be purchased',
        status: 400,
      })
    })

    test('when product does not exist', () => {
      productId = '3132a471-6931-4cf7-86a2-72d5e02dd5f0'
      return api
        .post(testEndpoint(productId))
        .set('Authorization', userAuth)
        .expect(404, { code: 'None', message: 'Not found', status: 404 })
    })

    test('returns 401 when user is not authorized', async () => {
      await api
        .post(testEndpoint(productId))
        .expect(401, { code: 'Unauthorized', message: 'Unauthorized', status: 401 })
    })
  })
})
