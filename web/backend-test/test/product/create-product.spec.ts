import {
  ICreateProductApiRequest,
  ICreateProductApiResponse,
  IGetProductApiResponse,
  PriceType,
  ProductCategory,
  SubscriptionInterval,
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

describe('Create Product', () => {
  const testEndpoint = '/api/products'
  let api: TestAgent
  let testHelperApiUrl: string
  let dbResetService: AppDbResetService
  let payload: ICreateProductApiRequest
  let adminAuth: string
  let userAuth: string

  beforeAll(() => {
    api = testagent(testConfig.get('apiUrl'))
    testHelperApiUrl = testConfig.get('apiTestHelperUrl')
    dbResetService = new AppDbResetService(testHelperApiUrl)
    adminAuth = adminAuthHeader()
  })

  beforeEach(async () => {
    await dbResetService.resetDb()
    userAuth = userAuthHeader('2199c39f-4fb3-4f72-b685-fe62b90fcef0')
    payload = {
      name: 'My New Product',
      description: 'Buy this great product, everybody loves it.',
      blurb: 'A very nice thing',
      category: ProductCategory.ArtDesign,
    }
  })

  const verifyProduct = async (
    id: string,
    auth: string,
  ): Promise<IGetProductApiResponse> => {
    const response = await api
      .get(`${testEndpoint}/${id}`)
      .set('Authorization', auth)
      .expect(200)

    const body: IGetProductApiResponse = response.body
    expect(body.name).toEqual(payload.name)
    expect(body.description).toEqual(payload.description)
    expect(body.blurb).toEqual(payload.blurb)
    expect(body.category).toEqual(payload.category)
    return body
  }

  test('admin creates product', async () => {
    const response = await api
      .post(testEndpoint)
      .set('Authorization', adminAuth)
      .send(payload)
      .expect(201)
    const body: ICreateProductApiResponse = response.body

    expect(body.id).toMatch(new RegExp(commonRegex.uuid))
    await verifyProduct(body.id, adminAuth)
  })

  test('user creates product', async () => {
    const response = await api
      .post(testEndpoint)
      .set('Authorization', userAuth)
      .send(payload)
      .expect(201)
    const body: ICreateProductApiResponse = response.body

    expect(body.id).toMatch(new RegExp(commonRegex.uuid))
  })

  test('user creates with price', async () => {
    payload.price = {
      name: 'Price1',
      price_type: PriceType.Subscription,
      amount: '500000000000',
      subscription_interval: SubscriptionInterval.Month,
      subscription_interval_count: 1,
      trial_days: 15,
    }
    const response = await api
      .post(testEndpoint)
      .set('Authorization', userAuth)
      .send(payload)
      .expect(201)
    const body: ICreateProductApiResponse = response.body

    expect(body.id).toMatch(new RegExp(commonRegex.uuid))
    const result = await verifyProduct(body.id, userAuth)
    expect(result.price?.name).toEqual(payload.price.name)
    expect(result.price?.price_type).toEqual(payload.price.price_type)
    expect(result.price?.amount).toEqual(payload.price.amount)
    expect(result.price?.subscription_interval).toEqual(
      payload.price.subscription_interval,
    )
    expect(result.price?.subscription_interval_count).toEqual(
      payload.price.subscription_interval_count,
    )
    expect(result.price?.trial_days).toEqual(payload.price.trial_days)
  })

  describe('when request is not valid', () => {
    test('returns 400 code when name length is invalid', async () => {
      // Name too short
      payload.name = 'a'
      await api.post(testEndpoint).set('Authorization', userAuth).send(payload).expect({
        status: 400,
        message: 'Failed to validate request',
        code: 'InvalidFormData',
      })

      // Name too long
      payload.name = '12345678901234567890123456789012345678901234567890zabcdefghijklm'
      return api.post(testEndpoint).set('Authorization', userAuth).send(payload).expect({
        status: 400,
        message: 'Failed to validate request',
        code: 'InvalidFormData',
      })
    })

    test('when description length is invalid', () => {
      payload.description = '1234'

      return api
        .post(testEndpoint)
        .set('Authorization', userAuth)
        .send(payload)
        .expect(400, {
          status: 400,
          message: 'Failed to validate request',
          code: 'InvalidFormData',
        })
    })

    test('when blurb length is invalid', () => {
      payload.blurb = 'a'

      return api
        .post(testEndpoint)
        .set('Authorization', userAuth)
        .send(payload)
        .expect(400, {
          status: 400,
          message: 'Failed to validate request',
          code: 'InvalidFormData',
        })
    })

    test('when category is invalid', () => {
      payload.category = 'a' as ProductCategory

      return api
        .post(testEndpoint)
        .set('Authorization', userAuth)
        .send(payload)
        .expect(400, {
          status: 400,
          message: 'Failed to deserialize the JSON body into the target type',
          code: 'InvalidFormData',
        })
    })
    test('returns 401 when user is not authorized', async () => {
      await api.post(testEndpoint).expect(401, {
        code: 'Unauthorized',
        message: 'Unauthorized',
        status: 401,
      })
    })
  })
})
