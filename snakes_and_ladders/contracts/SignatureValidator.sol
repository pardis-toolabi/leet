//SPDX-License-Identifier: MIT
pragma solidity ^0.8.10;

import "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import "hardhat/console.sol";

contract SignatureValidator {
    address public DEX;
    bytes4 internal constant MAGICVALUE = 0x1626ba7e;
    uint8 public threshold;
    uint8 public daoMemberCount;
    mapping(address => bool) public operationalAddresses;
    mapping(address => bool) public DaoMembers;

    constructor(
        address _dex,
        address[] memory _operationalAddresses,
        address[] memory _daoMembers,
        uint8 _threshold
    ) {
        DEX = _dex;
        threshold = _threshold;
        daoMemberCount = uint8(_daoMembers.length);
        for (uint256 i = 0; i < _operationalAddresses.length; ) {
            operationalAddresses[_operationalAddresses[i]] = true;

            unchecked {
                i++;
            }
        }

        for (uint256 i = 0; i < _daoMembers.length; ) {
            DaoMembers[_daoMembers[i]] = true;
            operationalAddresses[_daoMembers[i]] = true;

            unchecked {
                i++;
            }
        }
    }

    /**
     * @notice Splits signature bytes into `uint8 v, bytes32 r, bytes32 s`.
     * @dev Make sure to perform a bounds check for @param pos, to avoid out of          bounds access on @param signatures
     *      The signature format is a compact form of {bytes32 r}{bytes32 s}{uint8 v}
     *      Compact means uint8 is not padded to 32 bytes.
     * @param pos Which signature to read.
     *            A prior bounds check of this parameter should be performed, to avoid out of bounds access.
     * @param signatures Concatenated {r, s, v} signatures.
     * @return v Recovery ID or Safe signature type.
     * @return r Output value r of the signature.
     * @return s Output value s of the signature.
     */

    function signatureSplit(
        bytes memory signatures,
        uint256 pos
    ) internal pure returns (uint8 v, bytes32 r, bytes32 s) {
        // solhint-disable-next-line no-inline-assembly
        assembly {
            let signaturePos := mul(0x41, pos)
            r := mload(add(signatures, add(signaturePos, 0x20)))
            s := mload(add(signatures, add(signaturePos, 0x40)))
            /**
             * Here we are loading the last 32 bytes, including 31 bytes
             * of 's'. There is no 'mload8' to do this.
             * 'byte' is not working due to the Solidity parser, so lets
             * use the second best option, 'and'
             */
            v := and(mload(add(signatures, add(signaturePos, 0x41))), 0xff)
        }
    }

    function verifyDaoSignature(
        bytes32 dataHash,
        bytes memory signatures,
        uint8 _threshold
    ) internal view {
        require(signatures.length >= _threshold * 65, "ERROR: invalid signature length.");
        // There cannot be an owner with address 0.
        address lastOwner = address(0);
        address currentOwner;
        uint8 v;
        bytes32 r;
        bytes32 s;
        uint256 i;
        for (i = 0; i < _threshold; i++) {
            (v, r, s) = signatureSplit(signatures, i);
            currentOwner = ecrecover(dataHash, v, r, s);
            require(
                currentOwner > lastOwner && DaoMembers[currentOwner] == true,
                "ERROR: invalid signature"
            );      
            lastOwner = currentOwner;
        }
    }

    function isValidSignature(
        bytes32 _hash,
        bytes memory _signature
    ) public view returns (bytes4 magicValue) {
        require(msg.sender == DEX, "ERROR: unauthorized caller");
        address recoveredAddress = ECDSA.recover(_hash, _signature);
        if (operationalAddresses[recoveredAddress]) {
            return MAGICVALUE;
        } else {
            return 0xffffffff;
        }
    }

    function addMessagePrefix(
        bytes32 _messageHash    
    ) internal pure returns (bytes32) {
        return
            keccak256(
                abi.encodePacked(
                    "\x19Ethereum Signed Message:\n32",
                    _messageHash
                )
            );
    }
}