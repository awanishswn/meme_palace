# Memepalace

Welcome to the **Memepalace** project! Memepalace is a decentralized application (DApp) designed to manage and interact with memes on the blockchain. It allows users to post memes, like or dislike them, and fetch details about specific memes and their engagement metrics.

## Vision

The vision of **Memepalace** is to create a transparent, immutable platform for meme enthusiasts. By leveraging blockchain technology, we aim to provide a community-driven space where memes and their popularity are recorded and managed in a decentralized manner.

## Project Structure

### Contract Code

- **`Memepalace` Contract**:
  - **`post_meme`**: Posts a new meme to the platform. Generates a unique ID, stores the meme details with the IPFS CID, and updates the meme count.
  - **`like_meme`**: Increments the like count for a specified meme.
  - **`dislike_meme`**: Increments the dislike count for a specified meme.
  - **`fetch_meme`**: Retrieves details of a meme using its ID.
  - **`fetch_meme_overview`**: Retrieves the overview of a meme including likes and dislikes.

- **Types and Storage**:
  - **`Memeoverview`**: Contains information about meme ID, likes, and dislikes.
  - **`Meme`**: Stores meme details including ID and IPFS CID.
  - **`Memebook` and `Memeoverviewbook`**: Enums used for storing and fetching meme data from the blockchain storage.

### Dependencies

- **`soroban_sdk`**: The primary SDK used for building smart contracts on the Soroban platform.

### Storage Management

- **`COUNT_MEME`**: Symbol used to track the current number of memes posted.
- **`MEME_OV`**: Symbol for storing meme overview data.

## Usage

1. **Posting a Meme**: Call the `post_meme` function with the IPFS CID of the meme. This will create a new entry for the meme and increment the meme count.
2. **Liking or Disliking a Meme**: Use the `like_meme` or `dislike_meme` functions with the meme ID to update the like or dislike count.
3. **Fetching Data**: Retrieve meme details or overview using `fetch_meme` and `fetch_meme_overview` functions respectively.

## Running the Project

1. Ensure that the Soroban environment is set up.
2. Deploy the `Memepalace` contract to the Soroban blockchain.
3. Interact with the contract through transactions or a frontend interface integrated with Soroban.

## Contributing

We welcome contributions to the **Memepalace** project! Please fork the repository, make your changes, and submit a pull request. For detailed documentation and guidelines, please refer to the project's [CONTRIBUTING.md](CONTRIBUTING.md) file.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contact

For any questions or further information, please contact the maintainers via the project's issue tracker or by email.

Thank you for your interest in **Memepalace**!

