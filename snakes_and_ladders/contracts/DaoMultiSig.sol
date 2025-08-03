// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

// import "hardhat/console.sol";

import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/security/Pausable.sol";
import "./SignatureValidator.sol";

contract DaoMultiSig is Pausable, SignatureValidator, ReentrancyGuard {
    using SafeERC20 for IERC20;
    bytes32 DomainSeperatorHash;
    uint256 public nonce;

    // address zero is used to set a thrshold for ETH
    mapping(address => uint256) public withdrawalThreshold;
    mapping(address => bool) public authorizedWallets;

    modifier isOperator() {
        require(operationalAddresses[msg.sender], "ERROR: unauthorized caller");
        _;
    }

    modifier isDaoMember() {
        require(DaoMembers[msg.sender], "ERROR: unauthorized caller");
        _;
    }

    modifier isValidNonce(uint256 _nonce) {
        require(_nonce == nonce + 1, "ERROR: invalid nonce.");
        _;
    }

    modifier isAthorizedWallet(address _receiver) {
        require(authorizedWallets[_receiver], "ERROR: unauthorized receiver");
        _;
    }

    constructor(
        address[] memory _daoMembers,
        address[] memory _operationalAddresses,
        address[] memory _authorizedWallets,
        address _dex,
        uint8 _threshold
    ) SignatureValidator(_dex, _operationalAddresses, _daoMembers, _threshold) {
        DomainSeperatorHash = keccak256(
            abi.encodePacked(address(this), block.chainid)
        );
        for (uint256 i = 0; i < _authorizedWallets.length; ) {
            authorizedWallets[_authorizedWallets[i]] = true;
            unchecked {
                i++;
            }
        }
    }

    /**
     * @notice setDexAddress function is used to update the DEX variable,
     *
     * @dev the msg.sender should be one of the operators,
     * @dev the verifySignature functin will revert in case a signature is invalid,,
     * @dev the zero check for the new DEX address will be done by the front-end of the project
     * for gas reduction purposes,
     *
     * @param _nonce, is the nonce of the wallet to keep track of wallets transactions,
     * @param _proposedDex, is the new address to be set for the DEX variable,
     * @param _signature, contains the concatenated signature of DAO members gathered by the front-end,
     *
     */
    function setDexAddress(
        uint256 _nonce,
        bytes memory _signature,
        address _proposedDex
    ) external isOperator isValidNonce(_nonce) {
        bytes4 functionIdentifierHash = bytes4(
            keccak256("setDexAddress(uint256,bytes,address)")
        );
        bytes32 _hash = addMessagePrefix(
            keccak256(
                abi.encodePacked(
                    DomainSeperatorHash,
                    _nonce,
                    functionIdentifierHash,
                    _proposedDex
                )
            )
        );
        verifyDaoSignature(_hash, _signature, threshold);
        require(DEX != _proposedDex, "ERROR: invalid Address.");
        DEX = _proposedDex;
        nonce++;
    }

    /**
     * @notice addAuthorizedWallet function is used to update the authorizedWallet mapping,
     *
     * @dev the msg.sender should be one of the operators,
     * @dev the verifySignature functin will revert in case a signature is invalid,
     * @dev the zero check for the new authorizedWallet address will be done by the front-end of the project
     * for gas reduction purposes,
     *
     * @param _nonce, is the nonce of the wallet to keep track of wallets transactions,
     * @param _proposedWallet, is the new address to be added to authorizedWallets mapping,
     * @param _signature, contains the concatenated signature of DAO members gathered by the front-end,
     *
     */
    function addAuthorizedWallet(
        uint256 _nonce,
        bytes memory _signature,
        address _proposedWallet
    ) external isOperator isValidNonce(_nonce) {
        bytes4 functionIdentifierHash = bytes4(
            keccak256("addAuthorizedWallet(uint256,bytes,address)")
        );
        bytes32 _hash = addMessagePrefix(
            keccak256(
                abi.encodePacked(
                    DomainSeperatorHash,
                    _nonce,
                    functionIdentifierHash,
                    _proposedWallet
                )
            )
        );
        verifyDaoSignature(_hash, _signature, threshold);
        require(!authorizedWallets[_proposedWallet], "ERROR: invalid Address.");
        authorizedWallets[_proposedWallet] = true;
        nonce++;
    }

    /**
     * @notice removeAuthorizedWallet function is used to update the authorizedWallets mapping,
     *
     * @dev the msg.sender should be one of the operators,
     * @dev the verifySignature functin will revert in case a signature is invalid,
     * @dev the zero check for the new authorizedWallet address will be done by the front-end of the project
     * for gas reduction purposes,
     *
     * @param _nonce, is the nonce of the wallet to keep track of wallets transactions,
     * @param _proposedWallet, is the new address to be removed from authorizedWallets mapping ,
     * @param _signature, contains the concatenated signature of DAO members gathered by the front-end,
     *
     */
    function removeAuthorizedWallet(
        uint256 _nonce,
        bytes memory _signature,
        address _proposedWallet
    ) external isOperator isValidNonce(_nonce) {
        bytes4 functionIdentifierHash = bytes4(
            keccak256("removeAuthorizedWallet(uint256,bytes,address)")
        );
        bytes32 _hash = addMessagePrefix(
            keccak256(
                abi.encodePacked(
                    DomainSeperatorHash,
                    _nonce,
                    functionIdentifierHash,
                    _proposedWallet
                )
            )
        );
        verifyDaoSignature(_hash, _signature, threshold);
        require(authorizedWallets[_proposedWallet], "ERROR: invalid Address.");
        authorizedWallets[_proposedWallet] = false;
        nonce++;
    }

    /**
     * @notice setSignatureThreshold function is used to update the threshold variable,
     *
     * @dev the msg.sender should be one of the operators,
     * @dev the verifySignature functin will revert in case a signature is invalid,,
     * @dev the zero check for the new threshold will be done by the front-end of the project
     * for gas reduction purposes,
     *
     * @param _nonce, is the nonce of the wallet to keep track of wallets transactions,
     * @param _proposedThreshold, is the new threshold to be set to the variable,
     * @param _signature, contains the concatenated signature of DAO members gathered by the front-end,
     *
     */
    function setSignatureThreshold(
        uint256 _nonce,
        bytes memory _signature,
        uint8 _proposedThreshold
    ) external isOperator isValidNonce(_nonce) {
        require(
            _proposedThreshold <= daoMemberCount &&
                threshold != _proposedThreshold,
            "ERROR: invalid threshold amount."
        );
        bytes4 functionIdentifierHash = bytes4(
            keccak256("setSignatureThreshold(uint256,bytes,uint8)")
        );
        bytes32 _hash = addMessagePrefix(
            keccak256(
                abi.encodePacked(
                    DomainSeperatorHash,
                    _nonce,
                    functionIdentifierHash,
                    _proposedThreshold
                )
            )
        );
        verifyDaoSignature(_hash, _signature, threshold);

        threshold = _proposedThreshold;
        nonce++;
    }

    /**
     * @notice addOperator function is used to add another addresses to the operationalAddresses mapping,
     *
     * @dev the msg.sender should be one of the operators,
     * @dev the verifySignature functin will revert in case a signature is invalid,,
     * @dev the zero check for the input operator address will be done by the front-end of the project
     * for gas reduction purposes,
     *
     * @param _nonce, is the nonce of the wallet to keep track of wallets transactions,
     * @param _proposedOperators, is the address list to add to operators,
     * @param _signature, contains the concatenated signature of DAO members gathered by the front-end,
     *
     */

    function addOperator(
        uint256 _nonce,
        bytes memory _signature,
        address[] memory _proposedOperators
    ) external isOperator isValidNonce(_nonce) {
        bytes4 functionIdentifierHash = bytes4(
            keccak256("addOperator(uint256,bytes,address[])")
        );
        bytes32 _hash = addMessagePrefix(
            keccak256(
                abi.encodePacked(
                    DomainSeperatorHash,
                    _nonce,
                    functionIdentifierHash,
                    _proposedOperators
                )
            )
        );
        verifyDaoSignature(_hash, _signature, threshold);
        uint8 operatorsLength = uint8(_proposedOperators.length);
        for (uint256 i = 0; i < operatorsLength; ) {
            require(
                operationalAddresses[_proposedOperators[i]] == false,
                "ERROR: invalid input"
            );
            operationalAddresses[_proposedOperators[i]] = true;
            unchecked {
                i++;
            }
        }
        nonce++;
    }

    /**
     * @notice removeOperator function is used to remove addresses from the operationalAddresses mapping,
     *
     * @dev the msg.sender should be one of the operators,
     * @dev the verifySignature functin will revert in case a signature is invalid,,
     * @dev the zero check for the input operator address will be done by the front-end of the project
     * for gas reduction purposes,
     *
     * @param _nonce, is the nonce of the wallet to keep track of wallets transactions,
     * @param _proposedOperators, is the address list to remove from the operators ,
     * @param _signature, contains the concatenated signature of DAO members gathered by the front-end,
     *
     */
    function removeOperator(
        uint256 _nonce,
        bytes memory _signature,
        address[] memory _proposedOperators
    ) external isOperator isValidNonce(_nonce) {
        bytes4 functionIdentifierHash = bytes4(
            keccak256("removeOperator(uint256,bytes,address[])")
        );
        bytes32 _hash = addMessagePrefix(
            keccak256(
                abi.encodePacked(
                    DomainSeperatorHash,
                    _nonce,
                    functionIdentifierHash,
                    _proposedOperators
                )
            )
        );
        verifyDaoSignature(_hash, _signature, threshold);
        uint8 operatorsLength = uint8(_proposedOperators.length);
        for (uint256 i = 0; i < operatorsLength; ) {
            require(
                operationalAddresses[_proposedOperators[i]] == true,
                "ERROR: invalid input"
            );
            operationalAddresses[_proposedOperators[i]] = false;
            unchecked {
                i++;
            }
        }
        nonce++;
    }

    /**
     * @notice addDaoMembers function is used to add another address to the DaoMembers mapping,
     *
     * @dev the msg.sender should be one of the operators,
     * @dev the verifySignature functin will revert in case a signature is invalid,,
     * @dev the zero check for the input DAO address will be done by the front-end of the project
     * for gas reduction purposes,
     *
     * @param _nonce, is the nonce of the wallet to keep track of wallets transactions,
     * @param _proposedDAOMembers, is the address list to add to DAO members,
     * @param _signature, contains the concatenated signature of DAO members gathered by the front-end,
     *
     */

    function addDaoMembers(
        uint256 _nonce,
        bytes memory _signature,
        address[] memory _proposedDAOMembers
    ) external isOperator isValidNonce(_nonce) {
        bytes4 functionIdentifierHash = bytes4(
            keccak256("addDaoMembers(uint256,bytes,address[])")
        );
        bytes32 _hash = addMessagePrefix(
            keccak256(
                abi.encodePacked(
                    DomainSeperatorHash,
                    _nonce,
                    functionIdentifierHash,
                    _proposedDAOMembers
                )
            )
        );
        verifyDaoSignature(_hash, _signature, threshold);
        uint8 daoMembersLength = uint8(_proposedDAOMembers.length);
        for (uint256 i = 0; i < daoMembersLength; ) {
            require(
                DaoMembers[_proposedDAOMembers[i]] == false,
                "ERROR: invalid input."
            );
            DaoMembers[_proposedDAOMembers[i]] = true;
            operationalAddresses[_proposedDAOMembers[i]] = true;
            daoMemberCount++;
            unchecked {
                i++;
            }
        }
        nonce++;
    }

    /**
     * @notice removeDaoMembers function is used to remove addresses from the DaoMembers mapping,
     *
     * @dev the msg.sender should be one of the operators,
     * @dev the verifySignature functin will revert in case a signature is invalid,,
     * @dev the zero check for the input DAO address will be done by the front-end of the project
     * for gas reduction purposes,
     *
     * @param _nonce, is the nonce of the wallet to keep track of wallets transactions,
     * @param _proposedDAOMembers, is the address list to remove from DAO members,
     * @param _signature, contains the concatenated signature of DAO members gathered by the front-end,
     *
     */

    function removeDaoMembers(
        uint256 _nonce,
        bytes memory _signature,
        address[] memory _proposedDAOMembers
    ) external isOperator isValidNonce(_nonce) {
        require(
            (daoMemberCount - _proposedDAOMembers.length) >= threshold,
            "ERROR: invalid number of DAO."
        );
        bytes4 functionIdentifierHash = bytes4(
            keccak256("removeDaoMembers(uint256,bytes,address[])")
        );
        bytes32 _hash = addMessagePrefix(
            keccak256(
                abi.encodePacked(
                    DomainSeperatorHash,
                    _nonce,
                    functionIdentifierHash,
                    _proposedDAOMembers
                )
            )
        );
        verifyDaoSignature(_hash, _signature, threshold);
        uint8 daoMembersLength = uint8(_proposedDAOMembers.length);
        for (uint256 i = 0; i < daoMembersLength; ) {
            require(
                DaoMembers[_proposedDAOMembers[i]] == true,
                "ERROR: invalid input."
            );
            DaoMembers[_proposedDAOMembers[i]] = false;
            operationalAddresses[_proposedDAOMembers[i]] = false;
            daoMemberCount--;
            unchecked {
                i++;
            }
        }
        nonce++;
    }

    /**
     * @notice setWithdrawThreshold function is used to update the withdrawalThreshold mapping,
     *
     * @dev the msg.sender should be one of the operators,
     * @dev the verifySignature functin will revert in case a signature is invalid,,
     * @dev the zero check for the new threshold will be done by the front-end of the project
     * for gas reduction purposes,
     * @dev the zero check for the addresses will not be due to the fact that address(0) is considered as ETH
     *
     * @param _nonce, is the nonce of the wallet to keep track of wallets transactions,
     * @param _tokens, is the token list that thresholds will be applied on.
     * @param _proposedThresholds, is the threshold for the given token addresses,
     * @param _signature, contains the concatenated signature of DAO members gathered by the front-end,
     *
     */

    function setWithdrawThreshold(
        uint256 _nonce,
        bytes memory _signature,
        address[] memory _tokens,
        uint256[] memory _proposedThresholds
    ) external isOperator isValidNonce(_nonce) {
        bytes4 functionIdentifierHash = bytes4(
            keccak256("setWithdrawThreshold(uint256,bytes,address[],uint256[])")
        );
        bytes32 _hash = addMessagePrefix(
            keccak256(
                abi.encodePacked(
                    DomainSeperatorHash,
                    _nonce,
                    functionIdentifierHash,
                    _tokens,
                    _proposedThresholds
                )
            )
        );
        verifyDaoSignature(_hash, _signature, threshold);
        //function details
        uint256 tokenListLength = _tokens.length;
        require(
            tokenListLength == _proposedThresholds.length,
            "ERROR: invalid input"
        );

        for (uint256 i = 0; i < tokenListLength; ) {
            withdrawalThreshold[_tokens[i]] = _proposedThresholds[i];

            unchecked {
                i++;
            }
        }
        nonce++;
    }

    /**
     * @notice WithdrawNativeCoin function is used to withdraw  an specific amonut of Ether from the contract ,
     *
     * @dev the msg.sender should be one of the operators,
     * @dev the verifySignature functin will revert in case a signature is invalid,,
     * @dev the zero check for the amount and receiver addrerss will be done by the front-end of the project
     * for gas reduction purposes,
     *
     * @param _nonce, is the nonce of the wallet to keep track of wallets transactions,
     * @param _amount, is the amount of Ether to be withdrawn from the contract,
     * @param _receiver, the the address the Ether will be send to,
     * @param _signature, contains the concatenated signature of DAO members gathered by the front-end,
     *
     */

    function WithdrawNativeCoin(
        uint256 _nonce,
        bytes memory _signature,
        uint256 _amount,
        address _receiver
    ) external nonReentrant isOperator isValidNonce(_nonce) {
        bytes4 functionIdentifierHash = bytes4(
            keccak256("WithdrawNativeCoin(uint256,bytes,uint256,address)")
        );
        bytes32 _hash = addMessagePrefix(
            keccak256(
                abi.encodePacked(
                    DomainSeperatorHash,
                    _nonce,
                    functionIdentifierHash,
                    _amount,
                    _receiver
                )
            )
        );
        verifyDaoSignature(_hash, _signature, threshold);
        nonce++;
        payable(_receiver).transfer(_amount);
    }

    /**
     * @notice withdrawAllNativeCoin function is used to withdraw all of Ether from the contract ,
     *
     * @dev the msg.sender should be one of the operators,
     * @dev the verifySignature functin will revert in case a signature is invalid,,
     * @dev the zero check for the receiver addrerss will be done by the front-end of the project
     * for gas reduction purposes,
     *
     * @param _nonce, is the nonce of the wallet to keep track of wallets transactions,
     * @param _receiver, the the address the Ether will be send to,
     * @param _signature, contains the concatenated signature of DAO members gathered by the front-end,
     *
     */

    function withdrawAllNativeCoin(
        uint256 _nonce,
        bytes memory _signature,
        address _receiver
    ) external nonReentrant isOperator isValidNonce(_nonce) {
        bytes4 functionIdentifierHash = bytes4(
            keccak256("withdrawAllNativeCoin(uint256,bytes,address)")
        );
        bytes32 _hash = addMessagePrefix(
            keccak256(
                abi.encodePacked(
                    DomainSeperatorHash,
                    _nonce,
                    functionIdentifierHash,
                    _receiver
                )
            )
        );

        verifyDaoSignature(_hash, _signature, threshold);
        uint256 amount = address(this).balance;
        nonce++;
        payable(_receiver).transfer(amount);
    }

    /**
     * @notice withdrawToken function is used to withdraw an specific amonut of tokens from the contract,
     *
     * @dev the msg.sender should be one of the operators,
     * @dev the verifySignature functin will revert in case a signature is invalid,,
     * @dev the zero check for the receiver addrerss and token amounts will be done by the front-end of the project
     * for gas reduction purposes,
     *
     * @param _nonce, is the nonce of the wallet to keep track of wallets transactions,
     * @param _tokens, is the list of tokens addresses to be withdrawn from the contract,
     * @param _amounts, is the token amount to be withdrawn from the metioned token addresses,
     * @param _receiver, the the address the Ether will be send to,
     * @param _signature, contains the concatenated signature of DAO members gathered by the front-end,
     *
     */
    function withdrawToken(
        uint256 _nonce,
        bytes memory _signature,
        address[] memory _tokens,
        uint256[] memory _amounts,
        address _receiver
    ) external nonReentrant isOperator isValidNonce(_nonce) {
        bytes4 functionIdentifierHash = bytes4(
            keccak256(
                "withdrawToken(uint256,bytes,address[],uint256[],address)"
            )
        );
        bytes32 _hash = addMessagePrefix(
            keccak256(
                abi.encodePacked(
                    DomainSeperatorHash,
                    _nonce,
                    functionIdentifierHash,
                    _amounts,
                    _tokens,
                    _receiver
                )
            )
        );
        verifyDaoSignature(_hash, _signature, threshold);
        uint256 tokenListLength = _tokens.length;
        require(tokenListLength == _amounts.length, "ERROR: invalid input");
        nonce++;
        for (uint256 i = 0; i < tokenListLength; ) {
            IERC20(_tokens[i]).safeTransfer(_receiver, _amounts[i]);

            unchecked {
                i++;
            }
        }
    }

    /**
    /**
     * @notice withdrawAllTokens function is used to withdraw all of givrn tokens from the contract,
     *
     * @dev the msg.sender should be one of the operators,
     * @dev the verifySignature functin will revert in case a signature is invalid,,
     * @dev the zero check for the receiver addrersswill be done by the front-end of the project
     * for gas reduction purposes,
     *
     * @param _nonce, is the nonce of the wallet to keep track of wallets transactions,
     * @param _tokens, is the list of tokens addresses to be withdrawn from the contract,
     * @param _receiver, the the address the Ether will be send to,
     * @param _signature, contains the concatenated signature of DAO members gathered by the front-end,
     *
     */

    function withdrawAllTokens(
        uint256 _nonce,
        bytes memory _signature,
        address[] memory _tokens,
        address _receiver
    ) external nonReentrant isOperator isValidNonce(_nonce) {
        bytes4 functionIdentifierHash = bytes4(
            keccak256("withdrawAllTokens(uint256,bytes,address[],address)")
        );
        bytes32 _hash = addMessagePrefix(
            keccak256(
                abi.encodePacked(
                    DomainSeperatorHash,
                    _nonce,
                    functionIdentifierHash,
                    _tokens,
                    _receiver
                )
            )
        );
        verifyDaoSignature(_hash, _signature, threshold);
        uint256 tokenListLength = _tokens.length;
        nonce++;
        for (uint256 i = 0; i < tokenListLength; ) {
            uint256 _amount = IERC20(_tokens[i]).balanceOf(address(this));
            IERC20(_tokens[i]).safeTransfer(_receiver, _amount);

            unchecked {
                i++;
            }
        }
    }

    /**
     * @notice pause function is used to pause the wallet when needed,
     *
     * @dev the msg.sender should be one of the dao memebers,
     *
     */

    function pause() external whenNotPaused isDaoMember {
        _pause();
    }

    /**
     * @notice unpause function is used to unpause the contract,
     *
     * @dev the msg.sender should be one of the operators,
     * @dev the verifySignature functin will revert in case a signature is invalid,,
     *
     * @param _nonce, is the nonce of the wallet to keep track of wallets transactions,
     * @param _signature, contains the concatenated signature of DAO members gathered by the front-end,
     *
     */

    function unpause(
        uint256 _nonce,
        bytes memory _signature
    ) external nonReentrant isOperator isValidNonce(_nonce) {
        bytes4 functionIdentifierHash = bytes4(
            keccak256("unpause(uint256,bytes)")
        );
        bytes32 _hash = addMessagePrefix(
            keccak256(
                abi.encodePacked(
                    DomainSeperatorHash,
                    _nonce,
                    functionIdentifierHash
                )
            )
        );
        verifyDaoSignature(_hash, _signature, threshold);
        _unpause();
        nonce++;
    }
}
