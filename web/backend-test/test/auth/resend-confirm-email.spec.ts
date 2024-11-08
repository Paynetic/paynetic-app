import { AppDbResetService, testagent, TestAgent, userAuthHeader } from '../helpers'
import { testConfig } from '../test.config'
import { beforeAll, beforeEach, describe, test } from 'vitest'

describe('Resend confirm Email', () => {
  const testEndpoint = '/api/auth/resend-confirm-email'
  let api: TestAgent
  let testHelperApiUrl: string
  let dbResetService: AppDbResetService
  let userAuth: string

  beforeAll(() => {
    api = testagent(testConfig.get('apiUrl'))
    testHelperApiUrl = testConfig.get('apiTestHelperUrl')
    dbResetService = new AppDbResetService(testHelperApiUrl)
  })

  beforeEach(async () => {
    await dbResetService.resetDb()
  })

  test('return 200 when resend confirm email', () => {
    userAuth = userAuthHeader('90679368-ba27-4d7f-be85-849b4328d93a')
    return api.post(testEndpoint).set('Authorization', userAuth).expect(200)
  })

  test('return 400 when email is already confirmed', () => {
    userAuth = userAuthHeader('e10aa33c-94ef-4035-98d1-6372f82454d5')
    return api.post(testEndpoint).set('Authorization', userAuth).expect(400, {
      code: 'AlreadyConfirmed',
      message: 'Resend fail',
      status: 400,
    })
  })

  test('when user is not authorized', () => {
    return api.post(testEndpoint).expect(401, {
      code: 'Unauthorized',
      message: 'Unauthorized',
      status: 401,
    })
  })
})
