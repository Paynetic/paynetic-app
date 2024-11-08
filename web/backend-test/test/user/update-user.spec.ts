import {
  IGetUserApiResponse,
  IListUsersApiResponse,
  IRegisterUserApiRequest,
  IRegisterUserApiResponse,
  IUpdateUserApiRequest,
  UserType,
} from '@app/types'
import {
  testagent,
  TestAgent,
  adminAuthHeader,
  userAuthHeader,
  AppDbResetService,
  signMessage,
  TEST_ADDRESS1,
  TEST_PRIVATE_KEY1,
  registerSignature,
} from '../helpers'
import { testConfig } from '../test.config'
import { describe, expect, test, beforeAll, beforeEach } from 'vitest'
import { ethSwitchMessage } from '@app/util'

describe('Update User', () => {
  let api: TestAgent
  let testHelperApiUrl: string
  let dbResetService: AppDbResetService
  let payload: IUpdateUserApiRequest

  let adminAuth: string
  let userAuth: string

  let adminId: string
  let userId: string

  beforeAll(() => {
    api = testagent(testConfig.get('apiUrl'))
    testHelperApiUrl = testConfig.get('apiTestHelperUrl')
    dbResetService = new AppDbResetService(testHelperApiUrl)
  })

  beforeEach(async () => {
    adminAuth = adminAuthHeader()
    await dbResetService.resetDb()
    const response = await api.get(`/api/users`).set('Authorization', adminAuth)
    const body: IListUsersApiResponse = response.body
    const users = body.results

    adminId = users.find((user) => user.user_type === UserType.Admin)?.id || ''
    userId = users.find((user) => user.user_type === UserType.User)?.id || ''

    userAuth = userAuthHeader(userId)
  })

  describe('when requestor is Admin', () => {
    test('return 200 when updating user with type Admin', async () => {
      payload = { email: 'new@paynetic.net' }

      const response = await api
        .patch(`/api/users/${adminId}`)
        .set('Authorization', adminAuth)
        .send(payload)
        .expect(200)

      const body: IGetUserApiResponse = response.body

      expect(body.email).toEqual(payload.email)
      expect(body.email_confirmed).toBe(false)
    })

    test('return 200 when updating user with type User', async () => {
      payload = { user_type: 'Admin', user_status: 'Blocked' }

      const response = await api
        .patch(`/api/users/${userId}`)
        .set('Authorization', adminAuth)
        .send(payload)
        .expect(200)
      const body: IGetUserApiResponse = response.body

      expect(body.user_status).toEqual(payload.user_status)
    })
  })

  describe('when requestor is User', () => {
    test('return 200 when updating user with type User', async () => {
      payload = { email: 'new@paynetic.net' }

      const response = await api
        .patch(`/api/users/${userId}`)
        .set('Authorization', userAuth)
        .send(payload)
        .expect(200)
      const body: IGetUserApiResponse = response.body

      expect(body.email).toEqual(payload.email)
      expect(body.email_confirmed).toBe(false)
    })

    test('return 200 when updating eth_address', async () => {
      const message = ethSwitchMessage(TEST_ADDRESS1)
      const signature = await signMessage(TEST_PRIVATE_KEY1, message)
      payload = {
        eth_address: TEST_ADDRESS1,
        eth_address_signature: signature,
      }

      const response = await api
        .patch(`/api/users/${userId}`)
        .set('Authorization', userAuth)
        .send(payload)
        .expect(200)
      const body: IGetUserApiResponse = response.body

      expect(body.email).toEqual('user1@paynetic.net')
      expect(body.eth_address).toEqual(payload.eth_address)
    })

    test('return 200 when updating name', async () => {
      payload = { name: 'MyNewName' }

      const response = await api
        .patch(`/api/users/${userId}`)
        .set('Authorization', userAuth)
        .send(payload)
        .expect(200)
      const body: IGetUserApiResponse = response.body

      expect(body.email).toEqual('user1@paynetic.net')
      expect(body.name).toEqual(payload.name)
    })

    test('return 200 when updating description', async () => {
      payload = { description: 'My New Description' }

      const response = await api
        .patch(`/api/users/${userId}`)
        .set('Authorization', userAuth)
        .send(payload)
        .expect(200)
      const body: IGetUserApiResponse = response.body

      expect(body.email).toEqual('user1@paynetic.net')
      expect(body.description).toEqual(payload.description)
    })

    test('return 200 when updating link', async () => {
      payload = { link: 'https://newlink.com' }

      const response = await api
        .patch(`/api/users/${userId}`)
        .set('Authorization', userAuth)
        .send(payload)
        .expect(200)
      const body: IGetUserApiResponse = response.body

      expect(body.email).toEqual('user1@paynetic.net')
      expect(body.link).toEqual(payload.link)
    })

    test('return 200 when updating location', async () => {
      payload = { location: 'Test Location' }

      const response = await api
        .patch(`/api/users/${userId}`)
        .set('Authorization', userAuth)
        .send(payload)
        .expect(200)
      const body: IGetUserApiResponse = response.body

      expect(body.location).toEqual(payload.location)
    })

    test('return 400 when updating user type', async () => {
      payload = { user_type: 'Admin' }

      return api
        .patch(`/api/users/${userId}`)
        .set('Authorization', userAuth)
        .send(payload)
        .expect({
          code: 'None',
          message: 'Unauthorized field update',
          status: 400,
        })
    })

    test('return 400 when signature not included with eth_address', async () => {
      payload = { eth_address: '0x8628be3e373c3597C9643Ab587c964A061e70123' }

      await api
        .patch(`/api/users/${userId}`)
        .set('Authorization', userAuth)
        .send(payload)
        .expect({
          code: 'SignatureRequired',
          message: 'Signature required when updating eth_address',
          status: 400,
        })
    })

    test('return 400 when eth_address_signature is invalid', async () => {
      payload = {
        eth_address: '0x8628be3e373c3597C9643Ab587c964A061e70123',
        eth_address_signature:
          '0x8628be3e373c3597C9643Ab587c964A061e701230x8628be3e373c3597C9643Ab587c964A061e701230x8628be3e373c3597C9643Ab587c964A061e701230x8628be3e373c3597C9643Ab587c964A061e70123',
      }

      await api
        .patch(`/api/users/${userId}`)
        .set('Authorization', userAuth)
        .send(payload)
        .expect({
          code: 'InvalidSignature',
          message: 'Failed to parse signature',
          status: 400,
        })
    })

    test('return 403 when requestor is not the current user', async () => {
      const registerPayload: IRegisterUserApiRequest = {
        email: 'test@test.com',
        password: '12345678',
        eth_address: TEST_ADDRESS1,
        eth_address_signature: await registerSignature(TEST_PRIVATE_KEY1, TEST_ADDRESS1),
      }
      // add another user
      const response = await api
        .post('/api/users/registrations')
        .send(registerPayload)
        .expect(201)
      const body: IRegisterUserApiResponse = response.body

      payload = { email: 'test@paynetic.net' }

      return api
        .patch(`/api/users/${body.id}`)
        .set('Authorization', userAuth)
        .send(payload)
        .expect({
          code: 'None',
          message: `Forbidden`,
          status: 403,
        })
    })
  })

  test('returns 404 code when user id does not exist', () => {
    payload = { email: 'test@paynetic.net' }
    const id = 'cbd7a9ff-18f5-489e-b61e-cdd4a1394968'

    return api
      .patch(`/api/users/${id}`)
      .set('Authorization', userAuth)
      .send(payload)
      .expect({
        code: 'None',
        message: `User with ID ${id} not found`,
        status: 404,
      })
  })

  test('returns 400 code when email already exists', () => {
    // Try to create a user with the same email as a user from fixtures
    payload.email = 'admin1@paynetic.net'
    return api
      .patch(`/api/users/${adminId}`)
      .set('Authorization', adminAuth)
      .send(payload)
      .expect({
        code: 'UserExists',
        message: `Email ${payload.email} already exists`,
        status: 400,
      })
  })

  test('returns 400 code when old password does not matches current password', () => {
    payload = {
      old_password: '76543210',
      new_password: '1234567new',
    }

    return api
      .patch(`/api/users/${userId}`)
      .set('Authorization', userAuth)
      .send(payload)
      .expect({
        code: 'InvalidOldPassword',
        message: `Invalid old password`,
        status: 400,
      })
  })

  test('returns 400 code when new password is not valid', () => {
    payload = {
      old_password: 'password1',
      new_password: '7788',
    }

    return api
      .patch(`/api/users/${userId}`)
      .set('Authorization', userAuth)
      .send(payload)
      .expect({
        code: 'InvalidFormData',
        message: `Failed to validate request`,
        status: 400,
      })
  })

  test('returns 401 when user is not authorized', async () => {
    await api.get(`/api/users/${userId}`).expect(401, {
      code: 'Unauthorized',
      message: 'Unauthorized',
      status: 401,
    })
  })
})