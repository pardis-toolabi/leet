// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

// import "hardhat/console.sol";

import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/security/Pausable.sol";
import "./DaoMultiSig.sol";

// add error codes
contract Wallet is Pausable, DaoMultiSig {
    using SafeERC20 for IERC20;

    constructor(
        address[] memory _daoMembers,
        address[] memory _operationalAddresses,
        address[] memory _authorizedWallets,
        address _dex,
        uint8 _threshold
    )
        DaoMultiSig(
            _daoMembers,
            _operationalAddresses,
            _authorizedWallets,
            _dex,
            _threshold
        )
    {}

    /**
     * @notice approve function is used to authorize the operator to approve the DEX project to spend the contracts tokens,
     *
     * @dev the msg.sender should be one of the operators,
     * @dev the function will revert when the contract is paused,
     *
     * @param _tokens is the token list the operator will approve on,
     * @param _amount, is the amount the operator will approve
     *
     */
    function approve(
        IERC20[] memory _tokens,
        uint256 _amount
    ) external whenNotPaused nonReentrant isOperator {
        uint256 tokensLength = _tokens.length;
        for (uint256 i = 0; i < tokensLength; ) {
            SafeERC20.safeIncreaseAllowance(_tokens[i], DEX, _amount);
            unchecked {
                i++;
            }
        }
    }

    /**
     * @notice withdrawTokens function is used to authorize the operator to withdraw erc20 tokens to the authorizedWallet,
     *
     * @dev the msg.sender should be one of the operators,
     * @dev the function will revert when the contract is paused,
     * @dev the zero check for the amount list will be done by the front-end of the project
     * for gas reduction purposes
     *
     * @param _tokens is the token list the operator will withdraw from,
     * @param _amounts, is the amount the operator will withdraw,
     * @param _receiver, should be one of the authorized wallets,
     */
    function withdrawTokens(
        address[] memory _tokens,
        uint256[] memory _amounts,
        address _receiver
    ) external whenNotPaused nonReentrant isOperator isAthorizedWallet(_receiver){
        uint256 tokenListLength = _tokens.length;
        for (uint256 i = 0; i < tokenListLength; ) {
            if (withdrawalThreshold[_tokens[i]] == 0) {
                IERC20(_tokens[i]).safeTransfer(_receiver, _amounts[i]);
            } else {
                uint256 tokenWithdrawalThreshold = withdrawalThreshold[_tokens[i]];
                require(
                    _amounts[i] <= tokenWithdrawalThreshold,
                    "ERROR: amount exceeds the threhsold."
                );
                withdrawalThreshold[_tokens[i]] = tokenWithdrawalThreshold - _amounts[i];
                IERC20(_tokens[i]).safeTransfer(_receiver, _amounts[i]);
            }

            unchecked {
                i++;
            }
        }
    }

    /**
     * @notice withdrawNativeCoin function is used to authorize the operator to withdraw Ether to the authorizedWallet,
     *
     * @dev the msg.sender should be one of the operators,
     * @dev the function will revert when the contract is paused,
     * @dev the zero check for the amount list will be done by the front-end of the project
     * for gas reduction purposes,
     *
     * @param _amount, is the amount the operator will withdraw,
     * @param _receiver, should be one of the authorized wallets,
     *
     */
    function withdrawNativeCoin(uint256 _amount,
        address _receiver) external nonReentrant whenNotPaused isOperator isAthorizedWallet(_receiver) {
        if (withdrawalThreshold[address(0)] == 0) {
            payable(_receiver).transfer(_amount);
        } else {
            uint256 ethWithdrawalThreshold = withdrawalThreshold[address(0)];
            require(
                _amount <= ethWithdrawalThreshold,
                "ERROR: amount exceeds the threhsold."
            );
            withdrawalThreshold[address(0)] = ethWithdrawalThreshold - _amount;
            payable(_receiver).transfer(_amount);
        }
    }

    receive() external payable {}
}