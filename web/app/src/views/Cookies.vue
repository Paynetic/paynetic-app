<template>
  <div class="cookies-wrap">
    <img class="page-bg" :src="HeroBg" />
    <div class="blog-content">
      <h1 class="cookies-title blog-title">Cookie Policy</h1>
      <div class="blog-label">Last Updated: Nov. 4, 2024</div>
      <div class="blog-text">
        This Cookie Policy explains how Paynetic ("we," "us," or "our") uses cookies and
        similar technologies on our website,
        <a href="https://paynetic.net">https://paynetic.net</a> to provide, personalize,
        and enhance your experience. By using our site, you agree to the use of cookies as
        described in this policy.
      </div>
      <div class="blog-label">1. What Are Cookies?</div>
      <div class="blog-text">
        Cookies are small text files that are stored on your device when you visit a
        website. They help websites remember information about your visit, making it
        easier to use the site and enhancing your experience. Cookies can be "persistent"
        (stored on your device until they expire or are deleted) or "session" cookies
        (deleted when you close your browser).
      </div>
      <div class="blog-label">2. How We Use Cookies</div>
      <div class="blog-text">
        We use cookies and similar tracking technologies for various purposes, including:
        <ul>
          <li>
            <b>Essential Cookies: </b> These cookies are necessary for the functioning of
            our website. They enable basic functions like page navigation and access to
            secure areas of the site. Without these cookies, certain services may not be
            available.
          </li>
          <li>
            <b>Analytics Cookies: </b> These cookies help us understand how users interact
            with our site by collecting and reporting information anonymously. This data
            helps us improve website performance and user experience.
          </li>
          <li>
            <b>Functionality Cookies: </b> These cookies allow us to remember your
            preferences and settings, providing a more personalized experience on our
            website.
          </li>
          <li>
            <b>Advertising Cookies: </b> We may use cookies to deliver relevant ads and
            track the effectiveness of our marketing campaigns. These cookies may track
            your browsing behavior on our site and other websites.
          </li>
        </ul>
      </div>
      <div class="blog-label">3. Third-Party Cookies</div>
      <div class="blog-text">
        We may allow third-party service providers to place cookies on our site to support
        analytics, advertising, and other website functionalities. These third parties may
        collect information about your online activities over time and across different
        websites.
      </div>
      <div class="blog-label">4. Managing Your Cookie Preferences</div>
      <div class="blog-text">
        You have the right to manage or disable cookies at any time. Here are some ways
        you can control cookies:
        <ul>
          <li>
            <b>Browser Settings: </b> You can adjust your browser settings to block or
            delete cookies. Note that if you disable cookies, certain features of our
            website may not function as intended.
          </li>
          <li>
            <b>Opt-Out Tools: </b> Some third parties provide opt-out tools to control the
            collection of data via cookies. For example, you can opt out of Google
            Analytics by installing their browser add-on. To learn more about managing
            cookies on your browser, visit
            <a href="https://www.allaboutcookies.org/" target="_blank">
              All About Cookies.
            </a>
          </li>
        </ul>
      </div>
      <div class="blog-label">5. Changes to This Cookie Policy</div>
      <div class="blog-text">
        We may update this Cookie Policy from time to time to reflect changes in our
        practices or legal requirements. Updates will be posted on this page, and your
        continued use of our website signifies acceptance of any changes.
      </div>
      <div class="blog-label">6. Contact Us</div>
      <div class="blog-text">
        If you have questions about this Cookie Policy or how we use cookies, please
        contact us at:<br /><br /><b> Paynetic LLC<br /> </b>
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
.cookies-title {
  font-size: 48px;
  padding: 110px 0 130px;
}
</style>
