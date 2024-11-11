import { IListProductsApiRequest, IListProductsApiResponse } from '@app/types'
import { testagent, TestAgent, adminAuthHeader, userAuthHeader } from '../helpers'
import { testConfig } from '../test.config'
import { AppDbResetService } from '../helpers'
import { describe, expect, test, beforeAll, beforeEach } from 'vitest'

const TotalProducts = 11

describe('List Products', () => {
  const testEndpoint = '/api/products'
  let api: TestAgent
  let testHelperApiUrl: string
  let dbResetService: AppDbResetService
  let query: IListProductsApiRequest
  let adminAuth: string

  beforeAll(() => {
    api = testagent(testConfig.get('apiUrl'))
    testHelperApiUrl = testConfig.get('apiTestHelperUrl')
    dbResetService = new AppDbResetService(testHelperApiUrl)
  })

  beforeEach(async () => {
    await dbResetService.resetDb()
    adminAuth = adminAuthHeader()
  })

  describe('when requester is Admin', () => {
    test('return 200 status code and products with default sorting', async () => {
      const response = await api
        .get(testEndpoint)
        .set('Authorization', adminAuth)
        .expect(200)
      const body: IListProductsApiResponse = response.body

      expect(body.total).toEqual(TotalProducts)
      expect(body.results.length).toEqual(TotalProducts)
    })

    test('returns 200 status and sites when filtering from 1 to 2', async () => {
      query = { from: 1, to: 2 }

      const response = await api
        .get(testEndpoint)
        .query(query)
        .set('Authorization', adminAuth)
        .expect(200)

      const body: IListProductsApiResponse = response.body
      const users = body.results

      expect(body.total).toEqual(TotalProducts)
      expect(users.length).toEqual(2)

      expect(users[0].id).toEqual('2f46f1b9-0a24-4a68-bbc7-a26895a54e4d')
      expect(users[1].id).toEqual('c55864f1-8a47-4de2-9dd7-80d06479e70f')
    })

    test('filters by one category', async () => {
      const response = await api
        .get(testEndpoint)
        .query('categories[]=Technology')
        .set('Authorization', adminAuth)
        .expect(200)

      const body: IListProductsApiResponse = response.body

      expect(body.total).toEqual(2)
      expect(body.results[0].id).toEqual('c55864f1-8a47-4de2-9dd7-80d06479e70f')
      expect(body.results[1].id).toEqual('d105b284-e516-42c1-8690-6612e7cd7bc7')
    })

    test('filters by one status', async () => {
      const response = await api
        .get(testEndpoint)
        .query('statuses[]=Active')
        .set('Authorization', adminAuth)
        .expect(200)

      const body: IListProductsApiResponse = response.body

      expect(body.total).toEqual(3)
      expect(body.results[0].id).toEqual('673fad5b-626a-4a74-8cbd-afd45c4cf7d3')
      expect(body.results[1].id).toEqual('7922b138-65dc-412e-87c1-059325574595')
      expect(body.results[2].id).toEqual('1342c40f-dada-4e12-af3d-6e073ad171a4')
    })

    test('filters by one status and category', async () => {
      const response = await api
        .get(testEndpoint)
        .query('categories[]=Music&statuses[]=Active')
        .set('Authorization', adminAuth)
        .expect(200)

      const body: IListProductsApiResponse = response.body

      expect(body.total).toEqual(1)
      expect(body.results[0].id).toEqual('673fad5b-626a-4a74-8cbd-afd45c4cf7d3')
    })

    test('filters by multiple status and category', async () => {
      const response = await api
        .get(testEndpoint)
        .query(
          'categories[]=Fashion&categories[]=Technology&statuses[]=Active&statuses[]=Review',
        )
        .set('Authorization', adminAuth)
        .expect(200)

      const body: IListProductsApiResponse = response.body

      expect(body.total).toEqual(1)
      expect(body.results[0].id).toEqual('c55864f1-8a47-4de2-9dd7-80d06479e70f')
    })

    test('filters by user_id', async () => {
      query = { user_id: 'e10aa33c-94ef-4035-98d1-6372f82454d5' }
      const response = await api
        .get(testEndpoint)
        .query(query)
        .set('Authorization', adminAuth)
        .expect(200)

      const body: IListProductsApiResponse = response.body

      expect(body.total).toEqual(3)
      expect(body.results[0].id).toEqual('2f46f1b9-0a24-4a68-bbc7-a26895a54e4d')
      expect(body.results[1].id).toEqual('c55864f1-8a47-4de2-9dd7-80d06479e70f')
      expect(body.results[2].id).toEqual('26eb78aa-442f-4470-b724-d4ad090b9010')
    })
  })

  describe('when requester is User', () => {
    let userAuth: string

    beforeEach(() => {
      userAuth = userAuthHeader('e10aa33c-94ef-4035-98d1-6372f82454d5')
    })

    test('returns active and completed products with default sorting', async () => {
      const response = await api
        .get(testEndpoint)
        .set('Authorization', userAuth)
        .expect(200)
      const body: IListProductsApiResponse = response.body

      expect(body.total).toEqual(5)
      expect(body.results.length).toEqual(5)
      expect(body.results[0].id).toEqual('cf8550ca-3542-4165-8435-3f5c6c77a761')
      expect(body.results[1].id).toEqual('673fad5b-626a-4a74-8cbd-afd45c4cf7d3')
      expect(body.results[2].id).toEqual('7922b138-65dc-412e-87c1-059325574595')
      expect(body.results[3].id).toEqual('d105b284-e516-42c1-8690-6612e7cd7bc7')
      expect(body.results[4].id).toEqual('1342c40f-dada-4e12-af3d-6e073ad171a4')
    })

    test('filters by own user_id', async () => {
      query = { user_id: 'e10aa33c-94ef-4035-98d1-6372f82454d5' }
      const response = await api
        .get(testEndpoint)
        .query(query)
        .set('Authorization', userAuth)
        .expect(200)

      const body: IListProductsApiResponse = response.body

      expect(body.total).toEqual(3)
      expect(body.results[0].id).toEqual('2f46f1b9-0a24-4a68-bbc7-a26895a54e4d')
      expect(body.results[1].id).toEqual('c55864f1-8a47-4de2-9dd7-80d06479e70f')
      expect(body.results[2].id).toEqual('26eb78aa-442f-4470-b724-d4ad090b9010')
    })

    test('filters by other user_id', async () => {
      query = { user_id: '90679368-ba27-4d7f-be85-849b4328d93a' }
      const response = await api
        .get(testEndpoint)
        .query(query)
        .set('Authorization', userAuth)
        .expect(200)

      const body: IListProductsApiResponse = response.body

      expect(body.total).toEqual(5)
      expect(body.results[0].id).toEqual('cf8550ca-3542-4165-8435-3f5c6c77a761')
      expect(body.results[1].id).toEqual('673fad5b-626a-4a74-8cbd-afd45c4cf7d3')
      expect(body.results[2].id).toEqual('7922b138-65dc-412e-87c1-059325574595')
      expect(body.results[3].id).toEqual('d105b284-e516-42c1-8690-6612e7cd7bc7')
      expect(body.results[4].id).toEqual('1342c40f-dada-4e12-af3d-6e073ad171a4')
    })
  })

  describe('when requester is Anonymous', () => {
    test('returns active and completed products with default sorting', async () => {
      const response = await api.get(testEndpoint).expect(200)
      const body: IListProductsApiResponse = response.body

      expect(body.total).toEqual(5)
      expect(body.results.length).toEqual(5)
      expect(body.results[0].id).toEqual('cf8550ca-3542-4165-8435-3f5c6c77a761')
      expect(body.results[1].id).toEqual('673fad5b-626a-4a74-8cbd-afd45c4cf7d3')
      expect(body.results[2].id).toEqual('7922b138-65dc-412e-87c1-059325574595')
      expect(body.results[3].id).toEqual('d105b284-e516-42c1-8690-6612e7cd7bc7')
      expect(body.results[4].id).toEqual('1342c40f-dada-4e12-af3d-6e073ad171a4')
    })

    test('filters by user_id without active or completed products', async () => {
      query = { user_id: 'e10aa33c-94ef-4035-98d1-6372f82454d5' }
      const response = await api
        .get(testEndpoint)
        .query(query)
        .set('Authorization', adminAuth)
        .expect(200)

      const body: IListProductsApiResponse = response.body

      expect(body.total).toEqual(3)
      expect(body.results[0].id).toEqual('2f46f1b9-0a24-4a68-bbc7-a26895a54e4d')
      expect(body.results[1].id).toEqual('c55864f1-8a47-4de2-9dd7-80d06479e70f')
      expect(body.results[2].id).toEqual('26eb78aa-442f-4470-b724-d4ad090b9010')
    })
  })
})
