<template>
  <div class="terms-wrap">
    <img class="page-bg" :src="HeroBg" />
    <div class="blog-content">
      <h1 class="terms-title blog-title">Terms of Service</h1>
      <div class="blog-label">Last Updated: Nov. 4, 2024</div>
      <div class="blog-text">
        Welcome to <a href="https://paynetic.net">https://paynetic.net</a> By using our
        platform, you agree to the terms and conditions outlined here. Please read these
        Terms of Service ("Terms") carefully before accessing or using our website,
        services, or platform.
      </div>
      <div class="blog-label">1. Introduction</div>
      <div class="blog-text">
        Paynetic (“we,” “us,” or “our”) provides a cryptocurrency payment platform that
        enables businesses to use smart contracts for collecting recurring payments. This
        decentralized payment solution is designed for easy integration and a reliable,
        convenient experience. By using Paynetic, you agree to comply with these Terms.
      </div>
      <div class="blog-label">2. Acceptance of Terms</div>
      <div class="blog-text">
        By accessing or using our platform, you agree to be bound by these Terms, as well
        as any applicable laws and regulations. If you do not agree to these Terms, please
        refrain from using our services.
      </div>
      <div class="blog-label">3. Description of Services</div>
      <div class="blog-text">
        Paynetic allows businesses to collect recurring payments via smart contracts on a
        blockchain. Users can pay for services and products in a decentralized manner,
        receiving reminders to top up their wallets to maintain service continuity.
        Paynetic aims to be a reliable, accountable, and convenient solution, compatible
        with existing recurring payment systems.
      </div>
      <div class="blog-label">4. User Responsibilities</div>
      <div class="blog-text">
        <ul>
          <li>
            <b>Account Security: </b>You are responsible for maintaining the
            confidentiality of your account credentials. Notify us immediately if you
            suspect unauthorized access or account misuse.
          </li>
          <li>
            <b>Top-Ups: </b>Users are responsible for ensuring sufficient funds in their
            wallets to cover recurring payments. Paynetic sends reminders, but service
            continuation depends on wallet balance.
          </li>
          <li>
            <b>Compliance: </b>You agree to comply with all relevant local and
            international laws regarding cryptocurrency use and recurring payment
            transactions.
          </li>
        </ul>
      </div>
      <div class="blog-label">5. Payment and Fees</div>
      <div class="blog-text">
        <ul>
          <li>
            <b>Platform Fees: </b>Paynetic may charge transaction fees, which will be
            disclosed before you initiate a transaction.
          </li>
          <li>
            <b>Third-Party Fees: </b>Additional fees may be incurred based on third-party
            providers, network fees, or wallet providers, and are the user’s
            responsibility.
          </li>
          <li>
            <b>No Refunds: </b>Payments made through Paynetic are non-refundable, unless
            otherwise required by applicable law or expressly stated by the service
            provider.
          </li>
        </ul>
      </div>
      <div class="blog-label">6. License and Use of Services</div>
      <div class="blog-text">
        We grant you a limited, non-exclusive, and revocable license to access and use
        Paynetic for the intended purpose of collecting or making recurring payments. This
        license does not permit resale, sublicensing, or unauthorized commercial use of
        Paynetic.
      </div>
      <div class="blog-label">7 Prohibited Uses</div>
      <div class="blog-text">
        Users are prohibited from:
        <ul>
          <li>Using Paynetic for illegal transactions or activities.</li>
          <li>
            Engaging in unauthorized access, hacking, or exploitation of any part of the
            platform.
          </li>
          <li>Impersonating another person or entity.</li>
        </ul>
        Users found engaging in these activities may have their accounts suspended or
        terminated and may face legal action.
        <br /><br />
        Paynetic is not responsible for any actions taken by third-party payment
        processors, including the suspension or withholding of funds due to non-compliance
        with their policies.
      </div>
      <div class="blog-label">8. Intellectual Property</div>
      <div class="blog-text">
        All content, trademarks, designs, and intellectual property on Paynetic are owned
        or licensed by us. You may not use or reproduce any part of the platform’s
        intellectual property without prior written permission.
      </div>
      <div class="blog-label">9. Disclaimers and Limitations of Liability</div>
      <div class="blog-text">
        As a user of Paynetic, you agree to the following:
        <ul>
          <li>
            <b>Service Disclaimer: </b> Paynetic provides services “as is” and does not
            guarantee uninterrupted access or compatibility with all third-party services
            or devices.
          </li>
          <li>
            <b>No Warranty: </b> We make no warranties of any kind, express or implied,
            about the reliability, accuracy, or availability of Paynetic. Users assume all
            risks associated with using the platform.
          </li>
          <li>
            <b>Liability Limitation: </b>To the maximum extent permitted by law, Paynetic
            is not liable for any indirect, incidental, or consequential damages arising
            from your use of the platform.
          </li>
        </ul>
      </div>
      <div class="blog-label">10. Indemnification</div>
      <div class="blog-text">
        We reserve the right to terminate or suspend access to Paynetic, without notice,
        for actions that violate these Terms or if deemed necessary for security or
        operational reasons.
      </div>
      <div class="blog-label">11. Termination</div>
      <div class="blog-text">
        Paynetic does not process or manage refunds directly. Refunds, if applicable, are
        handled through the third-party payment processors or cryptocurrency wallets used
        during the transaction. Any disputes between fundraisers and contributors must be
        resolved independently, and Paynetic is not responsible for mediating or resolving
        disputes related to refunds or fund distribution.
      </div>
      <div class="blog-label">12. Modifications to Terms</div>
      <div class="blog-text">
        Paynetic may update these Terms from time to time. Changes will be posted on our
        website, and your continued use of the platform constitutes acceptance of any
        modifications.
      </div>
      <div class="blog-label">13. Contact Information</div>
      <div class="blog-text">
        If you have any questions about these Terms, please contact us at:<br /><br /><b>
          Paynetic LLC</b
        ><br />
        info@paynetic.net
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import HeroBg from '../assets/img/hero-bg.jpg'
</script>

<style lang="postcss" scoped>
@import '../css/defines.postcss';

.page-bg {
  height: 340px;
}
.terms-title {
  font-size: 48px;
  padding: 110px 0 130px;
}
</style>
