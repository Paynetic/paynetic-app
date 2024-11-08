import {
  BlockchainStatus,
  IGetProductApiResponse,
  PaymentCurrency,
  ProductCategory,
  ProductStatus,
} from '@app/types'
import { testagent, TestAgent, AppDbResetService } from '../helpers'
import { describe, expect, test, beforeAll, beforeEach } from 'vitest'
import { adminAuthHeader, userAuthHeader } from '../helpers'
import { testConfig } from '../test.config'
import { commonRegex } from '@app/util'

describe('Get User', () => {
  const testEndpoint = '/api/products'
  let api: TestAgent
  let testHelperApiUrl: string
  let dbResetService: AppDbResetService
  let adminAuth: string
  let productId: string

  beforeAll(() => {
    api = testagent(testConfig.get('apiUrl'))
    testHelperApiUrl = testConfig.get('apiTestHelperUrl')
    dbResetService = new AppDbResetService(testHelperApiUrl)
    adminAuth = adminAuthHeader()
  })

  beforeEach(async () => {
    await dbResetService.resetDb()
    productId = '673fad5b-626a-4a74-8cbd-afd45c4cf7d3'
  })

  describe('when requester is Admin', () => {
    test('returns 200 and product', async () => {
      const response = await api
        .get(`${testEndpoint}/${productId}`)
        .set('Authorization', adminAuth)
        .expect(200)

      const body: IGetProductApiResponse = response.body
      expect(body.id).toEqual(productId)
      expect(body.user_id).toEqual('90679368-ba27-4d7f-be85-849b4328d93a')
      expect(body.name).toEqual('Subscription Tunes')
      expect(body.description).toContain('Receive 10 unique digital songs per month')
      expect(body.blurb).toEqual('Revive the Classics with our Subscription Tunes')
      expect(body.contract_address).toEqual('0x0000000000000000000000000000000000000000')
      expect(body.payout_address).toEqual('0x0000000000000000000000000000000000000000')
      expect(body.category).toEqual(ProductCategory.Music)
      expect(body.status).toEqual(ProductStatus.Active)
      expect(body.blockchain_status).toEqual(BlockchainStatus.None)
      expect(body.transaction_hash).toBeNull()
      expect(body.price).toEqual({
        active: true,
        amount: '7000000000000000',
        base_currency: 'Ethereum',
        id: '7f30f816-3990-4152-8693-9137e96331ec',
        name: 'Tunes',
        price_type: 'Subscription',
        subscription_interval: 'Month',
        subscription_interval_count: 1,
        trial_days: 15,
      })
      expect(body.created_at).toMatch(new RegExp(commonRegex.date))
      expect(body.updated_at).toMatch(new RegExp(commonRegex.date))
    })
  })

  describe('when requester is User', () => {
    let userAuth: string

    beforeEach(() => {
      userAuth = userAuthHeader('e10aa33c-94ef-4035-98d1-6372f82454d5')
      productId = '2f46f1b9-0a24-4a68-bbc7-a26895a54e4d'
    })

    test('returns 200 and product when getting own product', async () => {
      const response = await api
        .get(`${testEndpoint}/${productId}`)
        .set('Authorization', userAuth)
        .expect(200)
      const body: IGetProductApiResponse = response.body

      expect(body.id).toEqual(productId)
      expect(body.user_id).toEqual('e10aa33c-94ef-4035-98d1-6372f82454d5')
      expect(body.name).toEqual('Steamed Ham Deck')
      expect(body.created_at).toMatch(new RegExp(commonRegex.date))
      expect(body.updated_at).toMatch(new RegExp(commonRegex.date))
    })

    test('returns 200 and product when getting other active product', async () => {
      const otherProductId = '7922b138-65dc-412e-87c1-059325574595'
      const response = await api
        .get(`${testEndpoint}/${otherProductId}`)
        .set('Authorization', userAuth)
        .expect(200)
      const body: IGetProductApiResponse = response.body

      expect(body.id).toEqual(otherProductId)
      expect(body.user_id).toEqual('90679368-ba27-4d7f-be85-849b4328d93a')
      expect(body.name).toEqual('iPhone Repair')
      expect(body.created_at).toMatch(new RegExp(commonRegex.date))
      expect(body.updated_at).toMatch(new RegExp(commonRegex.date))
    })

    test('returns 403 error when user gets another non-active user product', async () => {
      const otherProductId = '31153d5e-5c78-4156-85ee-fad95a25047b'
      await api
        .get(`${testEndpoint}/${otherProductId}`)
        .set('Authorization', userAuth)
        .expect(403, {
          code: 'None',
          message: 'Forbidden',
          status: 403,
        })
    })
  })

  test('returns 200 and product when getting active product', async () => {
    productId = '1342c40f-dada-4e12-af3d-6e073ad171a4'
    const response = await api.get(`${testEndpoint}/${productId}`).expect(200)
    const body: IGetProductApiResponse = response.body

    expect(body.id).toEqual(productId)
    expect(body.user_id).toEqual('90679368-ba27-4d7f-be85-849b4328d93a')
    expect(body.name).toEqual('Community Donation')
    expect(body.created_at).toMatch(new RegExp(commonRegex.date))
    expect(body.updated_at).toMatch(new RegExp(commonRegex.date))
  })

  test('returns 403 error when getting a non-active product', async () => {
    const nonActiveId = 'c55864f1-8a47-4de2-9dd7-80d06479e70f'
    await api.get(`${testEndpoint}/${nonActiveId}`).expect(403, {
      code: 'None',
      message: 'Forbidden',
      status: 403,
    })
  })

  test('returns 404 when product does not exist', async () => {
    const noneExistId = '870aafc9-36e9-476a-b38c-c1aaaad9d9fe'
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
