use std::collections::HashSet;

fn main() {
    println!("Part 1 is {}", part1(input()));
    println!("Part 2 is {}", part2(input()));
}

fn part1(input: &'static str) -> u32 {
    input
        .trim()
        .lines()
        .map(|line| {
            let line = line.trim();

            let (one, two) = line.split_at(line.len() / 2);

            let one: HashSet<char> = one.chars().collect();
            let two: HashSet<char> = two.chars().collect();

            let intersection: Vec<&char> = one.intersection(&two).collect();

            if intersection.len() != 1 {
                panic!();
            }

            let char_intersection = intersection.first().unwrap();

            let mut result = char_intersection.to_digit(36).unwrap() - 9;
            if char_intersection.is_ascii_uppercase() {
                result += 26;
            }

            result
        })
        .sum()
}

fn part2(input: &'static str) -> u32 {
    0
}

#[test]
fn test() {
    assert_eq!(
        157,
        part1(
            "
            vJrwpWtwJgWrhcsFMMfFFhFp
            jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
            PmmdzqPrVvPwwTWBwg
            wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
            ttgJtRGJQctTZtZT
            CrZsJsPPZsGzwwsLwLmpwMDw
        "
        )
    );
    // assert_eq!(
    //     12,
    //     part2(
    //         "
    //         A Y
    //         B X
    //         C Z
    //     "
    //     )
    // );
}

fn input() -> &'static str {
    "
    gtZDjBcmpcDgpZcmmbgtdtqmCGVCGGsvhCFCCqvmCMMM
    JrhfzfLTNfJhPnhQnfzHfCFFQFSGvMFCGQFsQSMSVs
    TllTRrfNNlfzwhtZBZgtRDBp
    vMdwjZdjwjvjdTZZvCcQMGnQMQcbcgLLCL
    rsVhfmssPWzDVGCLJSbCgPLSQG
    lfWNDHDgfszFRTFtwwNjdv
    GLPqVqdVGCLCdczjMjzMfzld
    JnWQvJDmvWBtlMzhrzfHQgcz
    tDtJDDDDtWRRmBwJwWtpPRsGCGScLPGSqspNCS
    ChVzZzfNDzNJmBQfjjJfmH
    MrTMPMncGMJvPPvPWTbrMWvgmBgQwgdpwmdpdpjwpHQcdw
    SPvvvbqrFvMvZzJzsFVzVJNV
    mvBbvMFqbMMVVmtCBHpDdDPTDspdNWPDVP
    zjSfftcQtwtSfQSpNDppsNsjPNdRPP
    fgfStJShrgvvCLLv
    GmFnNNwbFFbhQQGQnGwwwfBgnMMqVDBZVVBMfMVzVz
    vWzRRHzTHcgfZDVfBgfH
    SSTvrvRcPpcvjFGwNGbNpbwQwz
    FFgbZZFZgFmpstLgmbtzqNrwVPlMPlSWWrMPNp
    QQhTvjhcvjjvTcTcTfCcSRwwWzwzPMrzWNNWVVhwrwWq
    GRQBfCRnGGTcDvBfGvffCCjnFZtFFgStJLbLHbFLJZdgmd
    pppdjcrMMRDJLJdRcwRDrwssqHGGDHsZHHsvBVtvmVHV
    nlCFWzGzzQFlSlhGWnPzFbSsBZmsssmVVmsBvnHqvNVqqm
    lFTTTCSQSTrdGJJLJG
    jpsGMgsmghQwQsMmhlQshjtTNTRTnFqRWnnqRfFnnt
    SLBCHrcvZHbSvSZrSvSWnfvVNvftVlFRTqnRTq
    JrzdZbBcHBCrrlHrrSsMgmGpJPDPQmpgQgPG
    cmcZHgwgMgHSLmtjLfWPNNrWBNfffp
    JTqGTsClHslVVRVCVGVJGnBrjdnnrdBNvjPNBNBrWvnW
    VVlQlqTFJlzzlsVGsRCZMthHDbwbFhgcbwHchg
    qgZjgjjbssqgsjlNqjhTtdrfQdTdWLLnDVfHtHWd
    zcGMBDDzcLnztfQQQz
    JSppJcBScMmMFFBRCpRCMmGlggvjhbhlNlglwbslCZjhDZ
    hvhmqcqwwcTBvvwQnRQnRnTRFzFzQz
    jWLPPtPsgMtpdLMLWllpgLLQFQhFJjnVrzFrVFhnRzJJrJ
    WPWffgtSdspdhSMdlSdtfBbHmSvqbNBCCmcBmcvcCH
    frVcrVcggfSZJfbbJvBd
    hwWQnwhWQmQmThTSsdvvSMBTBzcb
    wGnFFCGlQwntGtCtwntwDmFwRgLrHqNRqqcNNgRrHHLggCjp
    wRSwwHDMsRGHvNBNjTgvjgJD
    mcLcFCclWQWQpPQWVQcQcvvNJjrNBTrvgJgBvTRvCg
    VFPbQLchQLSRfbMtdHGH
    lfVrhnlRRqrJZVDJdHSWCvJCJSbj
    BFsgcgMNNQgSvbfCff
    ffNPcMtzqPlnmRGh
    ZJplFmRJmWRJRWmTJCvtTtnLCtndCqtqnr
    SQsVPQHBQZNSNSLCfSLrcLcrrr
    VMjPjbNMDsVHmRllmZpZWmjh
    LcTLRbJhhdhLJbbclfVvfWQVWFRWFFfq
    rZNttSNvtgsPPFsqBFPWQF
    HGCSmHrrwNnHGMLpDhbzzpmJJv
    VlSWzRtWSJqWdfhdqBdF
    mTDHsmmmcHpgrCgCrTsMMtqfsFNsZqfdMZMNbd
    TDcpvrpHCprCpHrmcQvTHgTQzSnLJnPPJlLzwJtRVJwLjJ
    vZSWZJZJFvhZldZHdvvlphZSNGNnmzwCPNHNHGNrrRHGCPmP
    bjfgcbjTQTFQBnGRRBCBNwBnCz
    csqscsbssQLsgQcLgLQLQTQpFdlhdvdZdpZWhJplShWWtq
    QgQvHnfflfBwQCfwlfglnQQccNcRqGGcjmcsGjddwdzsJc
    DhZbTLZTDMVTsRzsqsRjszTz
    FSZVtMLMMWbSgqSvPQlnpH
    MMPllnnBmfSHvBgCLf
    whZjGRJdjcNjjhRjCvgCfbSvCZLHfpZs
    RRWGWwNRWwhwclmrgFmngFPMWm
    VVHQGDGDGsdRrmZBQZRCVHZCNcSTTPMwwvTTwSSNqBqvgMvN
    nfhdLfjFnJpblLbJjWhtnjWPScNnwSTPTPqTvgngNNvSvS
    fpWljtpLjflfLfzlhZGQHZQVddHrrQRDRz
    VCHCjwCwMSZSqQzhhQqcWZJD
    GGGrFFgNRNNgmfnTdgmWQpczvPvQPWQJGDpzzc
    lgTttRTgmfNRntrTTngrCbjCwJCHjLBBHlMVMsbB
    szgPPlCblggVszhLmzvcvNrqpjNqmrqqpGvG
    wBQDtBfQDtFvLjjctLqTMr
    ZFWWdDLQFwSfDSBSQQBWnnnQVdbhgRVbsHzsshbClzzCVggb
    VpVsHVcqcMVMMNHpsspstbMqzBztJZTBBfJfzTvZfvWJWSTv
    mDDQgCQQQHdrwgSvZSmJJZvWfJJf
    drCjggDlPdgrlbjNcnhcHsbpsj
    cNNDRRpDcNcTpppsqHLQGLfRLvHzLH
    lFntJjtbFFlsmsjvnGqHWLfhfqzzQh
    sgPbjBJtPgbPJblblJgbgbwdBTwDCwpwrdZZVcCcDppc
    GGclMjLnnjCMchcChLMLcnnzRFJDZJSRSzzzzDSShszPRS
    VHgFQgwVwfNNpQVfHzQsPPPJDbmZbJDJbS
    HfNVWdHVvgHgVWVNppNWVHwTlvBFcClBCjcTLTlBnnLrTL
    GTLdlJhffQwDRvWLrp
    HVZVNjjsPqzNjNNmNgDWMrRQpWvWRHrDHBWp
    VCqVzjPjCpVqCVPCsbctcnblcGlTbGnlbFJf
    flHdfdBNdZcflBMjqMjBNfZQhvJbGvqvsshJQsJCJDWvvD
    gFTzRRpzRTwbgbLmtCvsJhWsChrWCrtWCC
    VzzzFbVRLPznmRBffPNBHNMdlZfl
    FFFMwCqJFFmrRwgnbLrL
    GpjGpQHQpfjdjDRnLrbrRQmJzzgg
    BphfhDcNcHNvPBvSqJMWJS
    NndbWpDBNbjvWLZqWsWQ
    JPFFTSPfgcMgftQQGjvTmsGqzssG
    gPgcfcVFgcHqSqVhbBCHlpbbpDlhDD
    FSdfWFTTBnjsDCjsmrrT
    pQzLRVLppLGcQjqbmVDJsChCvCbVsm
    qHLRGqqZzGjLqBNMFdnHlNlBFN
    DjqbfBTchDjqqCjjCTWNTbdzSVzGZQGBwZnQnVwpSSnQ
    ssJlPrtvMsRLrrJQGNZJSpZpGzSG
    rlFssHsvPRPMvFmtHvtqjhTgjbqhWqNmNqgDNh
    vcpnRqwwLLbvvcGpDQWDFSCgMrWWQWRR
    gtNfBfllrFlHrlrl
    ZPzftBmsNBNBPJBZPmZPNtmPdGLsqbwqpqcndVLLGpVGvqgV
    vRBfQqqBQPfbrFvPBvPbhLDVDVDQZVVtZtlWLLLt
    jcJmFFwnhJVZLWVl
    sHTcmNNHzncmcjmdsBCrBCPCrBBqCFrqzb
    bbZRnGmNnBGGMNRTgCmWWGGSrvSvFHvzFvFQDF
    LjwphpdPdLpLJVqfJrQzDzfrvQHSvDcQrQ
    DJphdwDsnmbZsTZM
    rdNrZNBSzSztnNzWCcNpHlMwlwHWlM
    QqLGLJvLjtvQWhgHgchHwHJw
    GtjTGtDRqvfLRGnrzsmZmfrVFBrV
    TdMhZrTTNvwphcLL
    WnnmffmDWnWPsPCJNpNcpNVNQp
    fsjbWfFFfnmmDsFDnnflSSdczlMdTHTzTTRRBdtT
    cMcPcMcwgWJMjWWhFWCCQCmqCFdh
    bSLVLblnNnLbVfnsbSbCChSQdChptpdqZrmCmZ
    DLGNfnGVDNDHbfzjRcRgqHMRBJPc
    HVFVlVHjzjjlCJjHjCjnvDrggrgLdqzddMqrzz
    SSfBTmtNdLqngvrm
    TwnNfPWWpBSBNtTHZCGlPHCQJHZHPV
    prvccpFQpMcQBwsvssshdwSTPD
    qbGHVbNJGqwdPgDrTsDJ
    fGbGqqlGGHflqLlzZBBrRcrtrZlp
    fCSPhltMBmPmbdgd
    DjvJJscvTsHHDbWzBWsWbdwgLB
    VVHDZvTppRcJVFFppvvRJDJqMSGqCtZdthttrnthSZMGCr
    ZcSrSdrhDjBDDCmZdZmZjhwVHwqVVsMwgswVVwMfhw
    PNvzTPNbnzcPbGQNJTvqwsWgVgVMMWpQqwgHpp
    JTPGPTzNttnbRTPlPtNNRlFrFmBcmDljjmBFSCmLZZBr
    mNvRRCVMtNRdFNtMtBHHprpHgJgJWwpBnprg
    LZDDlSLlTslDfbcpJJWndwcscnwr
    qdZZGSDhMVRCGtmC
    VGFjjgBShGdGzQczcGRG
    MppqCDfCMwfLDfvNmrtWstRcMPzRMRsRsPQS
    NwDCffLppbqqrqvTBngSbnBHglZllH
    vdllJVDzmVDVqvvWvdqJlcWrCsfCsfSSsSJfCSfQQCCbCQ
    jnTHZPZHMjZhMjTpHgMpgnbNqBstnfrtSSrBSNssCrfN
    LHLTFLjTMTTTwjHhpHTcwmDcWVDlvRDmvqwWlW
    rqQsSStdmsdLqlNNPGlGlV
    FpFpzJNTcHzRHRHlGwFVLFBLFGVvlw
    WCCjWRNJTJWhQhbhrbnd
    jsQjfrRTRwzSsRTgNchlnlhqcnlQmQ
    dFDtdFBDddHLJpVpHHtVbtHFCWlWlGlNlmGggNqgglmcchqb
    dLDHMVdLtBBDBFVJBFthtJHRTvsMSvsTrTSRvPPjPzSwRP
    CSPpSrLlrlPrPchLnSlbDbbRttDVhbGRDDJRtD
    fzfvmzTMmfsFszsHZsHMHVfwtbjBDDGjtRBjQQGGJb
    HmvmTFmqmTsHqzzzzdTsMMScndccdLppnLCSPcCLrVgr
    pfMflRnfrnjrpjnFzDpfDMmMLRTLZVTgLsvdZgLLZHSVWZRd
    tBGNhwPGcNBBWwZddsSTTPgVLPdT
    JwthtwbbhNBQhwhbBCrzpnprnWnprlzWlClD
    PPnZZjnFNDjlJJhtMddfTTdD
    QGLHFWvQJtzfpvCt
    swqSmmQWLQwFWLwwRcqNNBnnbgPqbPNbglVZ
    GCLSjjZGZhpvGtBgjJlnJDhhJMVDPnJlJP
    mNtQQwNzQRHWdJHnPTsddlln
    zQrfmbtNbcQcrzmrRBZqBcvpjSGLZGLZBB
    zGNzgsjDssvNbPlWJfJq
    RLMVSRMLhCLZSMZHDSJWvpcqfbfhvpJqcWPv
    dMVHLFHLZMLRLLFRHHHVZMgDTntgstGwznzGGnzjDFwG
    wCLCHLBwzBtQRLHLbNFFfdqdDqVrVfBN
    JGvljmgGZvMlfDRRnnnZnfND
    GppRlgJlSllSgjMsmllpTjcCLczWztPWPwwwzWThtcQh
    WvHbvvWnFHszDRSltcCctCFD
    gCmJmCCPTPqpgrZtjdRtDRplcSjS
    rJJrQPPJQmrmrhGTznCfLMMbfvWfbCWQ
    TqBWtTbFBNNRRtwQpJJvvvZPpTSQ
    fRMfsMssrGhSmMwSQvvZJm
    VggcVlsCgHnVFnndbbnR
    NdrSSWBNPPSWWHPPlwlLZHLZLMhjlLLH
    pVptMTgVTzLwZTzlbF
    qsRmRJtsMvMqgqgRvCdcSrWSPcWrDmmdBN
    nbJnfqWcmCMnSBSHwzWBsHHz
    dVpdvdppdptppDlvlHcczSgNcgww
    VGTdTVtGtRLFPTDbcfCmmcCQJQjcrT
    VTjrjrjTlTjQMdpGrWMSHvSG
    wnNJbDmttnwnhNwcJmNGdvWvMSfvMfhSSppSdp
    JznFnNsGnzzGFDJsFNmLgVVQZBlLZjQTLTjTls
    hpngHwcpWHgjjfhzTJBfBB
    RFFbFlQlSdRsbRQQMGPRGdSGjBvvNTvzZMBvjzBBTJTvMBBT
    GPSSPDDDFzGlGGRzLzGGPRWqnprcgCHwCHpwHWVcncLV
    LLlLGffQLPRThRwP
    MpZjbmznWqmqZznmzmpZqZnMRgPBCTPfgRTTwTjhwBPPghjP
    VnZpMsMMJnWsmnJpJmzrtFlGQFrHGvSvfHStNV
    MQqHMQPnqmpDdTLLRnDjsj
    NGFzwgtLBtFFGrrCtzgfgCNgSsdTDSSTsdssjDdSlZRjTSBs
    zCwNLthfrbCgzzhqhmccJPhQHVmV
    SndBVcgdqcRBRcdPBBcVcQTSSMLMlTssMNMWsHMsLQ
    GmJvZvhqpvZtNwwWLTTLwMMm
    JFJpzFGZqjvhGZcjBPcCBBPnnVBc
    rJWbqTvwvJNbPDPPvLcZvPDp
    QMnfBsjmFPLcHRDfPp
    lQlMlmtFsMMBstljlnGhtMhmGNqJqTcWNNbWdGwdNNJCrTrq
    LcjcNCQNQWDpRDjRTj
    vWvszVVSsBGWsTJRFHRJTTSTRJ
    vvGbtqbGVVBqtzbqvBdzVLWNLClwnwMLWlQNMfdPQP
    TWBZsWrjzZzWBrBsrrsTLNNJvFnJVmlSFFQnGpmnSJJS
    qdCggdqqqhhqwhRbCwbCPqhlJFPPGJQVvvvnpVVmPnnFvS
    ffgCfghDqDdCsGWZjTsLrsfW
    QzQSSQmzSsLQcLmrcsLzccgqCnwqCtZDnDnrZwgnqTTT
    hFRHHRPRPMtWPGVPRlMljRPCgWBBDTgJBgnwqTZDBZDWDB
    jPjPHRMjjvdjVFhdNfbsbbQfbcddmNtL
    jJlTqMqJtdztJqzcSJSlTdSlprLsRRHwcRRrsrHbrnnRHsHL
    VVVMWNNWmNmLnPLRHrLp
    NGhfvvVWBNfNNCNCQTMqjzgTQBSSSqll
    SSSRMRSRpnMRHLqWLfPlDGlGWldD
    hbNtlmvrNrsVDWsGPfPfqG
    jvbBNmvlJjRcCzHFppCJ
    hhWWPjnBGBGnjqBWSnhhsNLllLNcLczJcqcTlLTlfl
    FHvFFMHwdmvrDbwCbbvHwdHnZTMLzTNTczflJTZclzNLlLcJ
    HdFFvdDvpCDdrnwrGhBQhWRRpsjQWWQW
    sBsvtJtdRdjNbWWrTllqlNgg
    nSZSnPPZzMSnSlScWWWgrVWCrqgrWMWr
    lzSncQcLZLzlwDvtdDdFdFJJhHvJ
    lpsTLDlTtFtlWHPDvvgPfgMrQQJM
    zmNbzcNjzldjwmbdbhhjcjRgfwrgvMwMMSRJSvQQvrRf
    ZhjqcjzNhmzNqBqNznmcWHplCFGnpCtFsGWHHWsH
    ZPGQBFHFbhSrHqtfSrSr
    nMdznzzMDTnjMQrMWtrMptplqpqS
    wzjczJmccTJCmcVghZBJbPBQBbVh
    wLLMJbqSBBnnJhbvbFSSRRlztTrHzrrrrd
    QNNGVPjWPGVqltTHWCqCdH
    sjNGmmGVGgQNGDVmsVpgqQVpMDhvbLwMffZfhZbLnfLLLZwb
    gQLcQrMtBPdwSBsSlmBm
    TfCpTJnTbfqgsgwgppsSzp
    jVbvTnvWfJnJjjbfCjWWjrFPrLMtcDPgLMQQRtgZVF
    gwpHvpgwngGHcnvNvgnmsqCzmMzlfqmmqzHHCm
    JrdSLdBVPRDtRtPfPPzCJhjqmljzmmqszzsM
    SWLDDtVdrZWtSBRZfRcwgFGnpNFpnTnWnTvT
    rpcnHrwrhWccNZDDBBgBVCSW
    nmzFRRjFmmJQNDJC
    qznMlqGnzRtRGvqGFRPrdMhwTpTLfLcppLHp
    wthvbmhmChWMRJLJzngZpzLLNC
    SsdBVjSTjBdffBFfcSdVHfTrnDZGpQgNZHNnLZGpJngJGLng
    sSdTcdVScdcrccjcrBPrBSjcvmRRwlWPhwmqtgWhMPtmMMqR
    CJJBdBCrHdBhtRHctBQhRMrBwZpwZWNZNSNTwSNpQWpZsSSW
    LVFnvnbDjLsDPsPqFFvPvDnTzSTwNwPZpSmpSpgmgZWNTW
    LjlflbFjsvVlrHcrHtrfcChH
    tVLJGNRtfBBNGBrfrbzmfhPsrsPC
    DWWDQHQgllSFqFzcsJmzzSSzmrrs
    MJFQDgMqnHlDvFdGNBNNZGNVVvjV
    wnNwGCBBFNWBqjFBnLLGVDHhHmDPHvZTjTvTrPvD
    bMbttVScMJQtdgSgstbJRSPmrTHmHmrmmSDZlrPrPDhv
    cMbgpsbVbzbdRMRFWLqzBfLGwwwwfW
    JpSnGSGpbGgsWWPHJrdfsT
    MNsRqNNvMQDTLWHlffNHLN
    qqmtRzRvCRRQDqjqjDmsmRpZwSZbcwbnCcCSBBnSSnnC
    TWqlqpRqRptqlRhrmtGGzhbSrSdz
    VgsBVMvgVZfZvPsMVNvfZfvVbSPdhFPFhbzLhJdGFJmLhhhL
    QZgvZgvHwbwHbMsMRllRjDRDnQRqlRjl
    fsPQwnHnHLLfnBBnwwGtjTGRWTWTWwhV
    jblbdjZFDMbGllqTGTtVlq
    gmdMgZMbjpZDcrrDgdmszsPLpQfpBPPnNQNLLz
    HRsPPGMhLPMrnPchPSwStjbSttSvtHSqQw
    dfsCfpCJVJCvdFBFwStwjj
    gTNWmWfTNVZVJzZWpWJgTpfhnDrMnDclgDlDrDRnRcMLDs
    ZQZQJMqdwmZvqfPmwRjpBBjHjnshnjtt
    zcTPTLDTFWLGTrTSWPcDSSHjRlhRsDhHslslssBRljjj
    TrNFLbTWrGNZvmvVQPQV
    htfLgmtSLcTWNLcT
    slbHlBBGbqRsblBHvdNJJcjFFNBTVWWWcn
    bbQsHMMblHrMsGRqvQhwCTQCwtQCzSpfmS
    zmqdphmFmSpTzhdqhFmwjjGbtcvDbcGGjllGQjSP
    HJFrMCsVLrHRRMCNrVMVnctvstlGcQlPtGGjQtGlvP
    RFLHLVWrNgVJzwzwfgffwdfp
    vdMjSmMMpmMWhRpndRmZnhvHqLpGHcJGGGDLHHLGcfcLfc
    lPBwwrsCgLFggcqqLW
    TWszsWNBTNdmSRvjbZZT
    zFlBGpzzzLLNjBwPcwwmcNPfWNQn
    VHSHRJTJDSVVnmcVVPpWmpnf
    DMZHHrDHHrJrrZrShZsHGbMBbFgGjGCgjpFlBzzb
    FVMpsvTqvqMssVsWZSrqWFvwlGDGwQzwfwQQNLzDlwlZwf
    hPbgBHhJJcJPwCwDpNllCCHC
    pnjbBmjgbgmqtSmsTtsF
    DHZHmfTmCfjDZHMZmzffHHnQwwTBdQwbSdBGBQwhBQTQww
    cqstRFWNtLrNFwdVShlBSlhBRl
    StJWpLptNWLtJcpqPrFHDjZzzvnDDHPCZjPvvz
    hzffhGVGGhzRqTBLTqHL
    sFFFsMQlwJMsmrBFSNHTHNqrTS
    pbdsJMdJMJbwbmJJtbTtgnffGgVVChvD
    FvJnFnCpQTddSSmFdFpPPsVhppDjBzjDVhDV
    RgZMZbsgzlDPlhjb
    cHHHRgRZgfHHZGZfHZcLLHrrCrmJCmddrsvdJsmvFFQG
    dpJDdZwLnvdvFmFMmHjslMLH
    CGCztgPhWCWhzzzNNPGfrrWfmbbsmmHjFHDMsbHMsjFPjbHm
    rNQDGzzhCCfNrzrDzChTcZZvQcTRJpTwdvQpVc
    VpvNGhGHGNhHbPsbVbvfFtLCzSCFSBsCFSFCLB
    MlqJwTnrRRrRnMlQMHfHzHzWFWtmTzLWFC
    ljZDDHqqjqRbpNhjNNgcgc
    qrQtDzcQzbrcfdbqrQrthtscSsvpvnsSHpTpLpspmsSs
    CVwNNVRNBSHsLSFBTv
    CVVVNZjlVlGwlGlljNlWJVrrfqbPQQqHqJhhftbfDJqf
    lpmrPDPDjPlmWrVzPztZwFjtFbBnRtZbbcRL
    dnqJCCgQdNqbqRbRbBLt
    QGhGddGCTdMHNTGgshgJhzvSmWWPSsnprpPzWzsWlr
    hCJHTdJJNvTdSSNssjvfwgntwDgtgwDGCtZwtRRB
    mbllFmFMFbMVWWLpbpZwwBZTZnnVwnTggtDB
    MmzLQpFPTmPzHvfJNNzhNs
    dzgBwzlgrrBrVLLlwLBgBlgRScDMMDDswMsHZRGDsZGZmM
    HPfPbjCFJjCvfnnsjsDDcccmZsRSMc
    hCvHfWPPnvJhPWpqNNhqLqzLqLLd
    
    "
}
