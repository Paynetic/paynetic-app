import { I18nObject, SimpleI18n, getString, getArray, getRecord } from './util'

const fallback: I18nObject = {
  title: 'Paynetic',
  cancel: 'Cancel',
  confirm: 'Confirm',
  created: 'Created',
  login: 'Log In',
  edit: 'Edit',
  wallet: 'Wallet',
  logout: 'Log Out',
  email: 'Email',
  name: 'Name',
  bio: 'Bio',
  link: 'Link',
  location: 'Location',
  home: 'Home',
  password: 'Password',
  save: 'Save',
  step: 'STEP',
  view: 'View',
  not_found: 'Page Not Found',
  not_found_text: 'This page does not exist!',
  // Home
  seamless: '<span>Seamless </span>Subscription Payments<br>in Cryptocurrency',
  effortless:
    'Effortlessly collect recurring payments with Paynetic, an AI-powered plugin for automated cryptocurrency subscription billing. Security, transparency, and innovation — <span>redefined</span>.',
  simplified: 'Crypto Subscription Payments Simplified',
  automate:
    'Automate your subscription billing using Ethereum and other cryptocurrencies with the power of blockchain and AI for faster, secure payments.',
  get_started: 'Get Started for Free',
  transform: 'Transform your business with Paynetic’s AI-driven crypto payment solution.',
  monthly: 'Monthly Subscription',
  subscribe_with: 'Subscribe With',
  sign_up: 'Sign Up Now',
  why: 'Why Paynetic',
  why_title1: 'AI-Driven Automation',
  why_text1:
    'Our AI optimizes transactions, monitors for payment issues, and enhances customer engagement by predicting behavior.',
  why_title2: 'Smart Contracts',
  why_text2:
    'Automate payments securely using blockchain technology, ensuring complete transparency and reliability.',
  why_title3: 'Multi-Crypto Support',
  why_text3:
    'Accept Ethereum, ERC-20 tokens, and other major cryptocurrencies, making payment processing seamless for your global customers.',
  why_title4: 'Fraud Protection',
  why_text4:
    'Advanced AI-powered fraud detection continuously monitors and protects transactions.',
  why_title5: 'Scalable and Flexible',
  why_text5:
    'Whether you handle dozens or thousands of subscriptions, Paynetic scales to meet your business needs without compromising on performance.',
  how_it: 'How It Works',
  how_title: 'Simple Crypto Subscriptions in Three Steps',
  connect_wallets: 'Connect Wallets',
  how_connect:
    'Customers subscribe with their preferred cryptocurrency wallets like MetaMask.',
  automate_title: 'Automate Payments',
  automate_text:
    'Let blockchain smart contracts handle recurring transactions, with AI monitoring performance for efficiency and security.',
  integrate: 'Integrate Paynetic',
  integrate_text:
    'Add our plugin to your platform and easily configure your subscription plans.',
  integration_learn: 'Learn More About Integration >',
  clients_title: 'What Our Clients Say',
  testimonials: 'Testimonials',
  integrations: 'Integrations',
  integrations_title: 'Integrations for Every Platform',
  integrations_text:
    'Paynetic seamlessly connects with the tools you already use, including:',
  commerce: 'E-commerce',
  wallets: 'Wallets',
  tools: 'Business Tools',
  supported: 'See All Supported Integrations',
  scale: {
    ready: 'Ready to Scale Your Subscription Payments with Crypto?',
    text: 'Leverage the power of Paynetic’s AI-powered, blockchain-based subscription payment solutions.',
    start: 'Get Started Today',
  },
  integration: {
    label: 'Integration Paynetic',
    title: 'Integration',
  },
  features: {
    title: 'Features',
  },
  connect: {
    title: 'Welcome to <span>Paynetic<span>',
    text: 'Join our community and collect cryptocurrency subscriptions at the click of a button!',
    login: 'Log in / Register with MetaMask',
    secure: 'Securely access your account in just a few clicks.',
    start: 'Get Started',
    button: 'CONNECT',
    how: 'How It Works:',
    install:
      'If you don’t have MetaMask installed, <a href="https://metamask.io" target="_blank">download it here</a>.',
    click: 'Click "CONNECT" to set up your MetaMask wallet and create your account.',
    email: 'Add an email and password for account recovery.',
    register: 'Register',
    register_text:
      'Add an email to help recover your account if you lose your wallet (password must be 8 characters). You will be asked to sign a message with your wallet to prove ownership.',
    email_login: 'Email login',
    login_text:
      "Log in with your email and password. You'll need to connect your wallet to set up products.",
  },
  faq: {
    link: 'Frequently Asked Questions >',
    what1: 'Who can use Paynetic?',
    what1_a:
      "Paynetic is designed for businesses and individuals who want to leverage cryptocurrency for automated, recurring payments. It's ideal for subscription-based services, SaaS providers, and businesses seeking to adopt a more decentralized approach to payment collection",
    what2: 'How does Paynetic compare to traditional payment platforms like Stripe?',
    what2_a:
      'Paynetic provides all the functionality of traditional recurring payment solutions with the added benefits of blockchain. We use smart contracts to automate and secure transactions without relying on a central authority, which reduces fees, enhances transparency, and provides decentralized accountability.',
    feat1: 'How does the recurring payment system work on Paynetic?',
    feat1_a:
      'Our platform leverages Ethereum smart contracts to handle recurring payments. Businesses set up a payment schedule, and the smart contract ensures that payments are automatically collected from users’ wallets according to the terms. Users receive reminders to top up their wallets to ensure continuous service.',
    feat2:
      'What happens if a user’s wallet doesn’t have enough funds for a recurring payment?',
    feat2_a:
      'If the wallet balance is insufficient, Paynetic will notify the user to top up their wallet. If the payment cannot be completed by the due date, the user may experience a temporary interruption in service until the required funds are available in the wallet.',
    feat3: 'How secure is Paynetic?',
    feat3_a:
      'Security is a top priority. Our platform operates on the Ethereum blockchain, using smart contracts that are transparent, auditable, and tamper-proof. With Paynetic, businesses and users can trust that each transaction is executed exactly as programmed without risk of interference or fraud.',
    feat4: 'What is the PNT token, and how is it used on Paynetic?',
    feat4_a:
      'PNT is an ERC20 token specifically designed for transactions on the Paynetic platform. Users can hold PNT in their wallets to fund their recurring payments, and businesses can accept PNT for their products and services.',
    feat5: 'Do I need to use PNT, or can I pay with other cryptocurrencies?',
    feat5_a:
      'PNT is the primary token used for payments on Paynetic, as it allows us to maintain a secure, consistent transaction process. In the future, we may introduce options for integrating other cryptocurrencies as well, so stay tuned for updates!',
    feat6: 'How do I acquire PNT tokens?',
    feat6_a:
      'PNT tokens can be purchased on various cryptocurrency exchanges where it is listed. Simply acquire PNT, and transfer it to your Paynetic-connected wallet to begin using the platform’s services.',
    how1: 'How do I get started as a business?',
    how1_a:
      'To get started, create a Paynetic business account and follow the steps to set up your payment contract and define your pricing. Once configured, your customers will be able to subscribe to your services and begin making recurring payments seamlessly.',
    how2: 'Can users pay from anywhere in the world?',
    how2_a:
      'Yes, Paynetic enables borderless transactions, allowing users to pay from anywhere with an internet connection and a compatible cryptocurrency wallet. We aim to provide a truly global payment solution.',
    how3: 'Does Paynetic charge fees?',
    how3_a:
      'Paynetic charges minimal transaction fees to support platform maintenance and ensure continuous service. These fees are significantly lower than traditional payment platforms, allowing you to save on payment processing costs.',
    how4: 'What should I do if I encounter an issue with my payment?',
    how4_a:
      'If you experience any issues, please reach out to our support team through our Contact page. Our dedicated support staff is here to help resolve any issues and ensure smooth service for both businesses and users.',
    how5: 'Can I cancel my subscription with a business on Paynetic?',
    how5_a:
      'Yes, users have full control over their subscriptions. You can cancel your subscription at any time through your Paynetic account. After cancellation, no further payments will be deducted from your wallet.',
  },
  profile: {
    title: 'Profile',
    settings: 'Settings',
    edit: 'Edit Profile',
    email_text: 'Your email is used for purchase notifications and account recovery.',
    password_text: "A password can be used to log in if your wallet isn't available",
    wallet_text:
      'The Ethereum address used to make purchases and receive funds. Changing your wallet will not affect previous transactions.',
    update_email: 'Update Email',
    new_email: 'New email',
    update_password: 'Update Password',
    update_wallet: 'Update Wallet',
    old_password: 'Old password',
    new_password: 'New password',
    confirm_password: 'Confirm new password',
    switch: 'Connect a new MetaMask wallet to update your Ethereum address.',
    sign: 'Sign a message to confirm your address change',
  },
  footer: {
    copyright: '© 2024 Paynetic - All Rights Reserved.',
    title: 'Crypto Subscription Payments Simplified',
    text: 'Automate your subscription billing using Ethereum and other cryptocurrencies with the power of blockchain and AI for faster, secure payments.',
    resources: 'Resources',
    whitepaper: 'Whitepaper',
    faq: 'FAQ',
    about: 'About',
    links: 'Quick Links',
    browse: 'Browse Products',
    trust: 'Trust & Safety',
    terms: 'Terms',
    privacy: 'Privacy Policy',
    cookie: 'Cookie Policy',
  },
  errors: {
    missing_wallet: 'Wallet missing, please go back and reconnect.',
    password_length: 'Password must be at least 8 characters.',
    name_len: 'Name must be between 2 and 20 characters.',
    bio_len: 'Bio must be less than 400 characters.',
    confirm_password: 'Passwords do not match.',
    signer: 'Error connecting to MetaMask',
    InvalidSignature: 'Unable to verify signature, please try again.',
    EthAddressUnique: 'Address is already in use.',
    UserExists: 'Email or wallet is already in use.',
    InvalidFormData: 'Invalid input',
    None: 'Unknown error',
  },
}

export const i18n = new SimpleI18n(fallback)
export const ts = getString(i18n)
export const ta = getArray(i18n)
export const tr = getRecord(i18n)
