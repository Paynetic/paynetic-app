import {
  IUpdateProductApiResponse,
  IUpdateProductApiRequest,
  ProductCategory,
  ProductStatus,
} from '@app/types'
import {
  testagent,
  TestAgent,
  adminAuthHeader,
  userAuthHeader,
  AppDbResetService,
} from '../helpers'
import { testConfig } from '../test.config'
import { describe, expect, test, beforeAll, beforeEach } from 'vitest'

describe('Update Product', () => {
  let api: TestAgent
  let testHelperApiUrl: string
  let dbResetService: AppDbResetService
  let adminAuth: string
  let userAuth: string
  let productId: string
  let payload: IUpdateProductApiRequest

  beforeAll(() => {
    api = testagent(testConfig.get('apiUrl'))
    testHelperApiUrl = testConfig.get('apiTestHelperUrl')
    dbResetService = new AppDbResetService(testHelperApiUrl)
  })

  beforeEach(async () => {
    await dbResetService.resetDb()
    adminAuth = adminAuthHeader()
    userAuth = userAuthHeader('e10aa33c-94ef-4035-98d1-6372f82454d5')
    productId = 'c55864f1-8a47-4de2-9dd7-80d06479e70f'
  })

  describe('when requestor is Admin', () => {
    test('return 200 when updating product name', async () => {
      payload = { name: 'Hello Product!' }

      const response = await api
        .patch(`/api/products/${productId}`)
        .set('Authorization', adminAuth)
        .send(payload)
        .expect(200)

      const body: IUpdateProductApiResponse = response.body
      expect(body.name).toEqual(payload.name)
    })

    test('return 200 when updating product status', async () => {
      payload = { status: ProductStatus.Approved }

      const response = await api
        .patch(`/api/products/${productId}`)
        .set('Authorization', adminAuth)
        .send(payload)
        .expect(200)

      const body: IUpdateProductApiResponse = response.body
      expect(body.status).toEqual(payload.status)
    })

    test('return 200 when updating all product properties', async () => {
      payload = {
        name: 'New name',
        description: 'New description',
        blurb: 'New blurb',
        category: ProductCategory.Music,
      }

      const response = await api
        .patch(`/api/products/${productId}`)
        .set('Authorization', adminAuth)
        .send(payload)
        .expect(200)

      const body: IUpdateProductApiResponse = response.body
      expect(body.name).toEqual(payload.name)
      expect(body.description).toEqual(payload.description)
      expect(body.blurb).toEqual(payload.blurb)
      expect(body.category).toEqual(payload.category)
    })
  })

  describe('when requestor is User', () => {
    test('return 200 when updating all product properties', async () => {
      payload = {
        name: 'New name',
        description: 'New description',
        blurb: 'New blurb',
        category: ProductCategory.Music,
      }

      const response = await api
        .patch(`/api/products/${productId}`)
        .set('Authorization', userAuth)
        .send(payload)
        .expect(200)

      const body: IUpdateProductApiResponse = response.body
      expect(body.name).toEqual(payload.name)
      expect(body.description).toEqual(payload.description)
      expect(body.blurb).toEqual(payload.blurb)
      expect(body.category).toEqual(payload.category)
    })

    test('return 400 when changing fields in active status', async () => {
      userAuth = userAuthHeader('90679368-ba27-4d7f-be85-849b4328d93a')
      productId = '673fad5b-626a-4a74-8cbd-afd45c4cf7d3'
      payload = { name: 'NEW NAME' }

      await api
        .patch(`/api/products/${productId}`)
        .set('Authorization', userAuth)
        .send(payload)
        .expect(400, {
          code: 'ProductActive',
          message: 'Failed to validate request',
          status: 400,
        })
    })

    test('return 403 when user attempts to directly update status', async () => {
      payload = { status: ProductStatus.Active }

      await api
        .patch(`/api/products/${productId}`)
        .set('Authorization', userAuth)
        .send(payload)
        .expect(403, {
          code: 'None',
          message: 'Forbidden',
          status: 403,
        })
    })

    test('return 403 when requestor does not own product', async () => {
      const otherUserAuth = userAuthHeader('90679368-ba27-4d7f-be85-849b4328d93a')
      payload = { name: 'TEST_RENAME' }

      return api
        .patch(`/api/products/${productId}`)
        .set('Authorization', otherUserAuth)
        .send(payload)
        .expect({
          code: 'None',
          message: 'Forbidden',
          status: 403,
        })
    })
  })

  test('returns 404 code when product does not exist', () => {
    const id = 'cbd7a9ff-18f5-489e-b61e-cdd4a1394968'

    return api
      .patch(`/api/products/${id}`)
      .set('Authorization', adminAuth)
      .send(payload)
      .expect({
        code: 'None',
        message: `Product with ID ${id} not found`,
        status: 404,
      })
  })

  test('returns 401 when user is not authorized', async () => {
    await api.patch(`/api/products/${productId}`).expect(401, {
      code: 'Unauthorized',
      message: 'Unauthorized',
      status: 401,
    })
  })
})
