import {
  IGetSubscriptionApiResponse,
  IUpdateSubscriptionApiRequest,
  SubscriptionStatus,
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

describe('Update Subscription', () => {
  let api: TestAgent
  let testHelperApiUrl: string
  let dbResetService: AppDbResetService
  let adminAuth: string
  let userAuth: string
  let subscriptionId: string
  let payload: IUpdateSubscriptionApiRequest

  beforeAll(() => {
    api = testagent(testConfig.get('apiUrl'))
    testHelperApiUrl = testConfig.get('apiTestHelperUrl')
    dbResetService = new AppDbResetService(testHelperApiUrl)
  })

  beforeEach(async () => {
    await dbResetService.resetDb()
    adminAuth = adminAuthHeader()
    userAuth = userAuthHeader('e10aa33c-94ef-4035-98d1-6372f82454d5')
    subscriptionId = '4aff4df5-e17b-4a23-8d58-bc2d376ff022'
  })

  describe('when requestor is Admin', () => {
    test('return 200 when updating subscription status', async () => {
      payload = { status: SubscriptionStatus.Active }

      await api
        .patch(`/api/subscriptions/${subscriptionId}`)
        .set('Authorization', adminAuth)
        .send(payload)
        .expect(200)

      const getResponse = await api
        .get(`/api/subscriptions/${subscriptionId}`)
        .set('Authorization', adminAuth)
        .expect(200)
      const body: IGetSubscriptionApiResponse = getResponse.body
      expect(body.status).toEqual(payload.status)
    })
  })

  describe('when requestor is User', () => {
    test('return 400 when updating subscription status', async () => {
      payload = { status: SubscriptionStatus.Active }

      await api
        .patch(`/api/subscriptions/${subscriptionId}`)
        .set('Authorization', userAuth)
        .send(payload)
        .expect(403, {
          code: 'None',
          message: 'User can only cancel subscription',
          status: 403,
        })
    })

    test('return 403 when requestor does not own subscription', async () => {
      const otherUserAuth = userAuthHeader('90679368-ba27-4d7f-be85-849b4328d93a')
      payload = { status: SubscriptionStatus.Cancelled }

      return api
        .patch(`/api/subscriptions/${subscriptionId}`)
        .set('Authorization', otherUserAuth)
        .send(payload)
        .expect({
          code: 'None',
          message: 'Forbidden',
          status: 403,
        })
    })
  })

  test('returns 404 code when subscription does not exist', () => {
    const id = 'f135c3b4-9f1b-47b6-bcfb-49fca8918d73'

    return api
      .patch(`/api/subscriptions/${id}`)
      .set('Authorization', adminAuth)
      .send(payload)
      .expect({
        code: 'None',
        message: `Subscription with ID ${id} not found`,
        status: 404,
      })
  })

  test('returns 401 when user is not authorized', async () => {
    await api.patch(`/api/subscriptions/${subscriptionId}`).expect(401, {
      code: 'Unauthorized',
      message: 'Unauthorized',
      status: 401,
    })
  })
})
