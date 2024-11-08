<template>
  <div class="privacy-wrap">
    <img class="page-bg" :src="HeroBg" />
    <div class="blog-content">
      <h1 class="privacy-title blog-title">Privacy Policy</h1>
      <div class="blog-label">Last Updated: Nov. 4, 2024</div>
      <div class="blog-text">
        Paynetic values your privacy. This Privacy Policy outlines how we collect, use,
        share, and protect your information when you use our cryptocurrency payment
        platform <a href="https://paynetic.net">https://paynetic.net</a> (“Platform”) and
        associated services. By accessing or using Paynetic, you agree to this Privacy
        Policy.
      </div>
      <div class="blog-label">1. Information We Collect</div>
      <div class="blog-text">
        We collect information necessary to provide, maintain, and improve our services.
        This includes:
        <ul>
          <li>
            <b>Account Information: </b>To create and maintain your Paynetic account, we
            may collect information like your email address and basic profile information.
          </li>
          <li>
            <b>Transaction Data: </b>We record details related to your transactions on the
            Platform, including transaction amounts, wallet addresses, and transaction
            timestamps.
          </li>
          <li>
            <b>Usage Information: </b>We collect information about how you interact with
            our Platform, including IP address, device information, and log data.
          </li>
          <li>
            <b>Cookies and Tracking Technologies: </b>We use cookies and similar
            technologies to remember your preferences and improve your experience on our
            Platform.
          </li>
        </ul>
      </div>
      <div class="blog-label">2. How We Use Your Information</div>
      <div class="blog-text">
        Paynetic uses your information for purposes including:
        <ul>
          <li>
            <b>Service Delivery: </b> To provide, operate, and maintain the Platform and
            process transactions.
          </li>
          <li>
            <b>Communication: </b> To respond to your inquiries, notify you of platform
            updates, and send reminders to top up your wallet for recurring payments.
          </li>
          <li>
            <b>Security and Fraud Prevention: </b> To ensure the integrity and security of
            our Platform and prevent fraudulent activity.
          </li>
          <li>
            <b>Improvement and Optimization: </b> To analyze usage trends and improve the
            user experience, performance, and functionality of our services.
          </li>
        </ul>
      </div>
      <div class="blog-label">3. Information Sharing and Disclosure</div>
      <div class="blog-text">
        Paynetic respects your privacy and does not sell your personal data to third
        parties. We may share information in limited circumstances, including:
        <ul>
          <li>
            <b>Service Providers: </b> We may share information with trusted third-party
            providers who assist us in operating the Platform, such as cloud storage
            providers or payment processors. These providers are bound by confidentiality
            agreements and only process information as instructed.
          </li>
          <li>
            <b>Legal Compliance and Safety: </b> We may disclose information if required
            by law or to protect the rights, safety, and property of Paynetic, its users,
            or the public.
          </li>
          <li>
            <b>Business Transfers: </b> In the event of a merger, acquisition, or asset
            sale, we may transfer your information to the new owner, subject to the terms
            of this Privacy Policy.
          </li>
        </ul>
      </div>
      <div class="blog-label">4. Data Security</div>
      <div class="blog-text">
        We implement industry-standard security measures to protect your information.
        These measures include encryption, secure access protocols, and regular security
        assessments. However, no system is completely secure, and we cannot guarantee the
        absolute security of your data.
      </div>
      <div class="blog-label">5. Data Retention</div>
      <div class="blog-text">
        Depending on your location, you may have rights regarding your personal
        information, including: We retain your information only as long as necessary to
        fulfill the purposes outlined in this Privacy Policy or as required by law.
        Transaction records may be stored for legal and compliance reasons, while account
        information is retained until account deletion.
      </div>
      <div class="blog-label">6. Your Choices and Rights</div>
      <div class="blog-text">
        We retain your personal information for as long as necessary to fulfill the
        <ul>
          <li>
            <b>Access: </b> Request access to the personal information we hold about you.
          </li>
          <li><b>Correction: </b> Update or correct your account information.</li>
          <li>
            <b>Deletion: </b> Request deletion of your personal information, subject to
            legal or operational restrictions.
          </li>
          <li>
            <b>Opt-Out: </b> Adjust your communication preferences by following the
            opt-out instructions in our emails or contacting us directly.
          </li>
        </ul>
      </div>
      <div class="blog-label">7. Cookies and Tracking Technologies</div>
      <div class="blog-text">
        Paynetic uses cookies and similar tracking technologies to enhance your experience
        on our Platform. You can control your cookie preferences through your browser
        settings, though disabling cookies may impact your experience.
      </div>
      <div class="blog-label">8. International Data Transfers</div>
      <div class="blog-text">
        Paynetic may store and process data in locations outside of your country of
        residence, including jurisdictions that may have different data protection
        standards. We take appropriate steps to protect your information, regardless of
        where it is stored.
      </div>
      <div class="blog-label">9. Children’s Privacy</div>
      <div class="blog-text">
        Paynetic is not intended for individuals under 18, and we do not knowingly collect
        information from children. If we discover that a minor has provided us with
        personal data, we will delete it promptly.
      </div>
      <div class="blog-label">10. Changes to This Privacy Policy</div>
      <div class="blog-text">
        We may update this Privacy Policy from time to time. Changes will be posted on our
        website, and your continued use of the Platform signifies acceptance of the
        revised policy.
      </div>
      <div class="blog-label">11. Contact Us</div>
      <div class="blog-text">
        If you have questions or concerns about this Privacy Policy or your data privacy
        with Paynetic, please contact us at:<br /><br /><b> Paynetic LLC</b><br />
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
.privacy-title {
  font-size: 48px;
  padding: 110px 0 130px;
}
</style>
