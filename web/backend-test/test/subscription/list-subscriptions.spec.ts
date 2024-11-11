import { IListSubscriptionsApiRequest, IListSubscriptionsApiResponse } from '@app/types'
import { testagent, TestAgent, adminAuthHeader, userAuthHeader } from '../helpers'
import { testConfig } from '../test.config'
import { AppDbResetService } from '../helpers'
import { describe, expect, test, beforeAll, beforeEach } from 'vitest'

const TotalSubscriptions = 7

describe('List Subscriptions', () => {
  const testEndpoint = '/api/subscriptions'
  let api: TestAgent
  let testHelperApiUrl: string
  let dbResetService: AppDbResetService
  let query: IListSubscriptionsApiRequest
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
    test('return 200 status code and subscriptions with default sorting', async () => {
      const response = await api
        .get(testEndpoint)
        .set('Authorization', adminAuth)
        .expect(200)
      const body: IListSubscriptionsApiResponse = response.body

      expect(body.total).toEqual(TotalSubscriptions)
      expect(body.results.length).toEqual(TotalSubscriptions)
    })

    test('returns 200 status and sites when filtering from 1 to 2', async () => {
      query = { from: 1, to: 2 }

      const response = await api
        .get(testEndpoint)
        .query(query)
        .set('Authorization', adminAuth)
        .expect(200)

      const body: IListSubscriptionsApiResponse = response.body

      expect(body.total).toEqual(2)
      expect(body.results.length).toEqual(2)

      expect(body.results[0].id).toEqual('bad3ed40-d8c3-4b5a-bd7b-44f54c8b4022')
      expect(body.results[1].id).toEqual('886122be-ac6c-4101-955f-ba603bede6fc')
    })

    test('filters by one status', async () => {
      const response = await api
        .get(testEndpoint)
        .query('statuses[]=Active')
        .set('Authorization', adminAuth)
        .expect(200)

      const body: IListSubscriptionsApiResponse = response.body

      expect(body.total).toEqual(1)
      expect(body.results[0].id).toEqual('4aff4df5-e17b-4a23-8d58-bc2d376ff022')
    })

    test('filters by multiple status and user_id', async () => {
      const filter_user = 'e10aa33c-94ef-4035-98d1-6372f82454d5'
      const response = await api
        .get(testEndpoint)
        .query(`user_id=${filter_user}&statuses[]=Active&statuses[]=Initial`)
        .set('Authorization', adminAuth)
        .expect(200)

      const body: IListSubscriptionsApiResponse = response.body

      expect(body.total).toEqual(2)
      expect(body.results[0].id).toEqual('4aff4df5-e17b-4a23-8d58-bc2d376ff022')
      expect(body.results[1].id).toEqual('ebbd2025-62f6-4444-bc01-54ceea0d1fba')
    })

    test('filters by user_id', async () => {
      query = { user_id: 'e10aa33c-94ef-4035-98d1-6372f82454d5' }
      const response = await api
        .get(testEndpoint)
        .query(query)
        .set('Authorization', adminAuth)
        .expect(200)

      const body: IListSubscriptionsApiResponse = response.body

      expect(body.total).toEqual(3)
      expect(body.results[0].id).toEqual('bad3ed40-d8c3-4b5a-bd7b-44f54c8b4022')
      expect(body.results[1].id).toEqual('4aff4df5-e17b-4a23-8d58-bc2d376ff022')
      expect(body.results[2].id).toEqual('ebbd2025-62f6-4444-bc01-54ceea0d1fba')
    })
  })

  describe('when requester is User', () => {
    let userAuth: string

    beforeEach(() => {
      userAuth = userAuthHeader('e10aa33c-94ef-4035-98d1-6372f82454d5')
    })

    test('filters by own user_id', async () => {
      query = { user_id: 'e10aa33c-94ef-4035-98d1-6372f82454d5' }
      const response = await api
        .get(testEndpoint)
        .query(query)
        .set('Authorization', userAuth)
        .expect(200)

      const body: IListSubscriptionsApiResponse = response.body

      expect(body.total).toEqual(3)
      expect(body.results[0].id).toEqual('bad3ed40-d8c3-4b5a-bd7b-44f54c8b4022')
      expect(body.results[1].id).toEqual('4aff4df5-e17b-4a23-8d58-bc2d376ff022')
      expect(body.results[2].id).toEqual('ebbd2025-62f6-4444-bc01-54ceea0d1fba')
    })

    test('returns 403 when filtering by other user_id', async () => {
      query = { user_id: '2199c39f-4fb3-4f72-b685-fe62b90fcef0' }
      await api
        .get(testEndpoint)
        .query(query)
        .set('Authorization', userAuth)
        .expect(403, {
          code: 'None',
          message: 'User must filter by own ID',
          status: 403,
        })
    })
  })

  describe('when requester is Anonymous', () => {
    test('returns 401 unauthorized', async () => {
      await api.get(testEndpoint).expect(401, {
        code: 'Unauthorized',
        message: 'Unauthorized',
        status: 401,
      })
    })
  })
})
