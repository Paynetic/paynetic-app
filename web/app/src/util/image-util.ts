import { IProductViewModel } from '@app/types'
import Album from '../assets/img/placeholder-album.jpg'
import Tunes from '../assets/img/placeholder-tunes.jpg'
import iPhone from '../assets/img/placeholder-iphone.jpg'
import Tech from '../assets/img/placeholder-tech.jpg'
import Donation from '../assets/img/placeholder-donation.jpg'
import placeholder from '../assets/img/product-placeholder.jpg'

export const getMainImage = (project: IProductViewModel) => {
  return (
    {
      'cf8550ca-3542-4165-8435-3f5c6c77a761': Album,
      '673fad5b-626a-4a74-8cbd-afd45c4cf7d3': Tunes,
      '7922b138-65dc-412e-87c1-059325574595': iPhone,
      'd105b284-e516-42c1-8690-6612e7cd7bc7': Tech,
      '1342c40f-dada-4e12-af3d-6e073ad171a4': Donation,
    }[project.id] ?? placeholder
  )
}
